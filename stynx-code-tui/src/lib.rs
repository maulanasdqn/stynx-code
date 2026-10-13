pub mod clipboard;
pub mod dialogs;
pub mod event;
pub mod layout;
pub mod persistence;
pub mod render;
pub mod state;
pub mod theme;
pub mod tool_ui_impl;
pub mod util;
pub mod widgets;

pub use event::event_handler::{EventHandler, UiAction};
pub use render::renderer::Renderer;
pub use state::{
    AppState, ConversationState, DialogOption, DisplayMessage, DisplayToolUse,
    InputState, ModalKind, ModalState, PermissionChoice, SelectKind, ToolUseStatus,
};

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

use crossterm::{
    event::{
        DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
        KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

static KEYBOARD_ENHANCED: AtomicBool = AtomicBool::new(false);

fn push_keyboard_enhancement(out: &mut impl io::Write) {
    if crossterm::terminal::supports_keyboard_enhancement().unwrap_or(false)
        && execute!(out, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)).is_ok()
    {
        KEYBOARD_ENHANCED.store(true, Ordering::Relaxed);
    }
}

fn pop_keyboard_enhancement(out: &mut impl io::Write) {
    if KEYBOARD_ENHANCED.swap(false, Ordering::Relaxed) {
        let _ = execute!(out, PopKeyboardEnhancementFlags);
    }
}
use ratatui::{Terminal, backend::CrosstermBackend};

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub state: AppState,
    in_alt: bool,
}

pub fn restore_terminal() {
    pop_keyboard_enhancement(&mut io::stdout());
    let _ = disable_raw_mode();
    let _ = execute!(
        io::stdout(),
        DisableMouseCapture,
        DisableBracketedPaste,
        LeaveAlternateScreen,
    );
}

fn install_panic_hook() {
    use std::sync::Once;
    static INSTALLED: Once = Once::new();
    INSTALLED.call_once(|| {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            restore_terminal();
            prev(info);
        }));
    });
}

impl TuiApp {
    pub fn new() -> io::Result<Self> {
        install_panic_hook();
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture, EnableBracketedPaste)?;
        push_keyboard_enhancement(&mut stdout);
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal, state: AppState::new(), in_alt: true })
    }

    pub fn draw(&mut self) -> io::Result<()> {
        let Self { terminal, state, .. } = self;
        terminal.draw(|frame| Renderer::draw(frame, state))?;
        Ok(())
    }

    pub fn leave_alt(&mut self) {
        if self.in_alt {
            pop_keyboard_enhancement(self.terminal.backend_mut());
            disable_raw_mode().ok();
            execute!(self.terminal.backend_mut(), DisableMouseCapture, DisableBracketedPaste, LeaveAlternateScreen).ok();
            self.in_alt = false;
        }
    }

    pub fn enter_alt(&mut self) {
        if !self.in_alt {
            enable_raw_mode().ok();
            execute!(self.terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture, EnableBracketedPaste).ok();
            push_keyboard_enhancement(self.terminal.backend_mut());
            self.terminal.clear().ok();
            self.in_alt = true;
        }
    }

    pub fn is_in_alt(&self) -> bool { self.in_alt }

    pub fn frame_signature(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let s = &self.state;
        let mut h = std::collections::hash_map::DefaultHasher::new();
        s.spinner_frame.hash(&mut h);
        s.elapsed_secs.hash(&mut h);
        s.is_streaming.hash(&mut h);
        s.is_pending.hash(&mut h);
        s.is_paused.hash(&mut h);
        s.toasts.items.len().hash(&mut h);
        s.modal.active.is_some().hash(&mut h);
        s.conversation.messages.len().hash(&mut h);
        if let Some(last) = s.conversation.messages.last() {
            last.content.len().hash(&mut h);
            last.thinking.len().hash(&mut h);
            last.tool_uses.len().hash(&mut h);
        }
        s.live_thinking.len().hash(&mut h);
        s.sub_agents.len().hash(&mut h);
        for a in &s.sub_agents {
            a.step.hash(&mut h);
        }
        s.total_input.hash(&mut h);
        s.total_output.hash(&mut h);
        s.model_name.hash(&mut h);
        s.permission_mode.hash(&mut h);
        h.finish()
    }

    pub fn tick_spinner(&mut self) {
        if self.state.is_streaming || self.state.is_pending {
            self.state.spinner_tick = self.state.spinner_tick.wrapping_add(1);
            if self.state.spinner_tick.is_multiple_of(8) {
                self.state.spinner_frame = self.state.spinner_frame.wrapping_add(1) % 10;
            }
        }
        self.state.toasts.tick();
    }

    pub fn sync_pause(&mut self, paused: &AtomicBool) {
        if paused.load(Ordering::Relaxed) {
            self.leave_alt();
        } else if !self.in_alt {
            self.enter_alt();
        }
    }
}

impl Drop for TuiApp {
    fn drop(&mut self) { self.leave_alt(); }
}
