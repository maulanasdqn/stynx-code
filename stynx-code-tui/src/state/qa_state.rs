use serde::Deserialize;

const MARKER: &str = "@@QA@@";

#[derive(Deserialize, Clone)]
#[serde(from = "RawOption")]
pub struct QaOption {
    pub label: String,
    pub description: Option<String>,
}

// Models sometimes send an option as a bare string instead of an object.
#[derive(Deserialize)]
#[serde(untagged)]
enum RawOption {
    Label(String),
    Full {
        label: String,
        #[serde(default)]
        description: Option<String>,
    },
}

impl From<RawOption> for QaOption {
    fn from(raw: RawOption) -> Self {
        match raw {
            RawOption::Label(label) => Self { label, description: None },
            RawOption::Full { label, description } => Self { label, description },
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct QaQuestion {
    #[serde(default)]
    pub question: String,
    #[serde(default)]
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
    let mut envelope: Envelope = serde_json::from_str(json).ok()?;
    envelope.questions.retain(|q| !q.options.is_empty());
    for q in &mut envelope.questions {
        // Models sometimes leave out `question` or `header`; show the one that is present.
        if q.question.trim().is_empty() {
            q.question = q.header.clone();
        }
        if q.header.trim().is_empty() {
            q.header = q.question.chars().take(12).collect();
        }
    }
    (!envelope.questions.is_empty()).then_some(envelope.questions)
}

/// Plain-text form of a question payload, for the free-text fallback dialog.
pub fn readable_prompt(raw: &str) -> String {
    let Some(json) = raw.trim_start().strip_prefix(MARKER) else { return raw.to_string() };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return "The agent asked a question that could not be shown. Type your answer.".to_string();
    };
    let mut lines = Vec::new();
    for q in value["questions"].as_array().into_iter().flatten() {
        let title = q["question"].as_str().or(q["header"].as_str()).unwrap_or("Question");
        lines.push(title.to_string());
        for opt in q["options"].as_array().into_iter().flatten() {
            let label = opt.as_str().or(opt["label"].as_str()).unwrap_or_default();
            match opt["description"].as_str() {
                Some(desc) if !label.is_empty() => lines.push(format!("  • {label} — {desc}")),
                Some(desc) => lines.push(format!("  • {desc}")),
                None => lines.push(format!("  • {label}")),
            }
        }
    }
    lines.join("\n")
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
    fn tolerates_missing_question_and_string_options() {
        let raw = r#"@@QA@@{"questions":[{"header":"Fix","options":[{"label":"Edit README","description":"No code change"},"Split files"]}]}"#;
        let questions = parse_qa(raw).unwrap();
        assert_eq!(questions[0].question, "Fix");
        assert_eq!(questions[0].options[1].label, "Split files");
    }

    #[test]
    fn plain_text_is_not_qa() {
        assert!(parse_qa("What should I do next?").is_none());
    }
}
