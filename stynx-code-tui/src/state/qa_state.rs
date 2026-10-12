use serde::Deserialize;

const MARKER: &str = "@@QA@@";

#[derive(Deserialize, Clone)]
pub struct QaOption {
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct QaQuestion {
    pub question: String,
    pub header: String,
    #[serde(default, rename = "multiSelect")]
    pub multi_select: bool,
    pub options: Vec<QaOption>,
}

#[derive(Deserialize)]
struct Envelope {
    questions: Vec<QaQuestion>,
}

pub fn parse_qa(raw: &str) -> Option<Vec<QaQuestion>> {
    let json = raw.trim_start().strip_prefix(MARKER)?;
    let envelope: Envelope = serde_json::from_str(json).ok()?;
    (!envelope.questions.is_empty()).then_some(envelope.questions)
}

pub struct QaState {
    pub questions: Vec<QaQuestion>,
    pub index: usize,
    pub cursor: usize,
    pub picks: Vec<Vec<usize>>,
    pub other: Vec<String>,
}

pub enum QaOutcome {
    Pending,
    Done(String),
}

impl QaState {
    pub fn new(questions: Vec<QaQuestion>) -> Self {
        let n = questions.len();
        Self { questions, index: 0, cursor: 0, picks: vec![Vec::new(); n], other: vec![String::new(); n] }
    }

    pub fn current(&self) -> &QaQuestion {
        &self.questions[self.index]
    }

    pub fn other_index(&self) -> usize {
        self.current().options.len()
    }

    pub fn on_other(&self) -> bool {
        self.cursor == self.other_index()
    }

    pub fn is_answered(&self, index: usize) -> bool {
        let picks = &self.picks[index];
        let other = self.questions[index].options.len();
        !picks.is_empty() && (!picks.contains(&other) || !self.other[index].trim().is_empty())
    }

    pub fn move_cursor(&mut self, delta: isize) {
        let last = self.other_index() as isize;
        self.cursor = (self.cursor as isize + delta).clamp(0, last) as usize;
    }

    pub fn goto(&mut self, index: usize) {
        if index < self.questions.len() {
            self.index = index;
            self.cursor = self.picks[index].first().copied().unwrap_or(0);
        }
    }

    pub fn toggle(&mut self, option: usize) {
        self.cursor = option;
        let multi = self.current().multi_select;
        let picks = &mut self.picks[self.index];
        if multi {
            match picks.iter().position(|&p| p == option) {
                Some(at) => {
                    picks.remove(at);
                }
                None => picks.push(option),
            }
        } else {
            *picks = vec![option];
        }
    }

    pub fn type_other(&mut self, c: char) {
        self.other[self.index].push(c);
        let other = self.other_index();
        if !self.picks[self.index].contains(&other) {
            self.toggle(other);
        }
    }

    pub fn backspace_other(&mut self) {
        self.other[self.index].pop();
    }

    pub fn confirm(&mut self) -> QaOutcome {
        if !self.current().multi_select || self.picks[self.index].is_empty() {
            let cursor = self.cursor;
            if !self.current().multi_select || !self.picks[self.index].contains(&cursor) {
                self.toggle(cursor);
            }
        }
        if !self.is_answered(self.index) {
            return QaOutcome::Pending;
        }
        match (0..self.questions.len()).find(|&i| !self.is_answered(i)) {
            Some(next) => {
                self.goto(next);
                QaOutcome::Pending
            }
            None => QaOutcome::Done(self.answer()),
        }
    }

    pub fn answer(&self) -> String {
        self.questions
            .iter()
            .enumerate()
            .map(|(i, q)| {
                let picked: Vec<String> = self.picks[i]
                    .iter()
                    .map(|&p| match q.options.get(p) {
                        Some(option) => option.label.clone(),
                        None => self.other[i].trim().to_string(),
                    })
                    .collect();
                format!("{}: {}", q.header, picked.join(", "))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = r#"@@QA@@{"questions":[
        {"header":"Diff view","question":"Where?","options":[{"label":"Left"},{"label":"Right","description":"OpenCode style"}]},
        {"header":"Extras","question":"Pick any","multiSelect":true,"options":[{"label":"Stats"},{"label":"Paths"}]}
    ]}"#;

    #[test]
    fn parses_and_builds_answer() {
        let mut qa = QaState::new(parse_qa(RAW).unwrap());
        qa.move_cursor(1);
        assert!(matches!(qa.confirm(), QaOutcome::Pending));
        assert_eq!(qa.index, 1);
        qa.toggle(0);
        qa.toggle(1);
        match qa.confirm() {
            QaOutcome::Done(answer) => assert_eq!(answer, "Diff view: Right\nExtras: Stats, Paths"),
            QaOutcome::Pending => panic!("expected done"),
        }
    }

    #[test]
    fn other_requires_text() {
        let mut qa = QaState::new(parse_qa(RAW).unwrap());
        qa.cursor = qa.other_index();
        assert!(matches!(qa.confirm(), QaOutcome::Pending));
        assert_eq!(qa.index, 0);
        for c in "Bottom".chars() {
            qa.type_other(c);
        }
        assert!(matches!(qa.confirm(), QaOutcome::Pending));
        assert_eq!(qa.index, 1);
        assert!(qa.answer().starts_with("Diff view: Bottom"));
    }

    #[test]
    fn plain_text_is_not_qa() {
        assert!(parse_qa("What should I do next?").is_none());
    }
}
