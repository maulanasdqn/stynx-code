use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use portable_pty::{CommandBuilder, MasterPty, PtySize};

use super::shells::{default_shell, shell_args, zsh_keys_dir};
use super::utf8::Utf8Carry;

struct Session {
    master: Box<dyn MasterPty + Send>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
    writer: Box<dyn Write + Send>,
    zdot: Option<PathBuf>,
}

#[derive(Default)]
pub struct TerminalManager {
    sessions: HashMap<u32, Session>,
    next_id: u32,
}

fn size(cols: u16, rows: u16) -> PtySize {
    PtySize { rows: rows.max(2), cols: cols.max(2), pixel_width: 0, pixel_height: 0 }
}

impl TerminalManager {
    pub fn open<O, E>(
        &mut self,
        shell: Option<String>,
        cwd: &Path,
        cols: u16,
        rows: u16,
        on_output: O,
        on_exit: E,
    ) -> Result<u32, String>
    where
        O: Fn(String) + Send + 'static,
        E: FnOnce() + Send + 'static,
    {
        self.next_id += 1;
        let id = self.next_id;
        let shell = shell.filter(|s| !s.trim().is_empty()).unwrap_or_else(default_shell);
        let pair = portable_pty::native_pty_system().openpty(size(cols, rows)).map_err(|e| e.to_string())?;

        let mut cmd = CommandBuilder::new(&shell);
        cmd.args(shell_args(&shell));
        cmd.cwd(cwd);
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        cmd.env("TERM_PROGRAM", "Stynx");
        let zdot = zsh_keys_dir(&shell, id);
        if let Some(dir) = &zdot {
            cmd.env("ZDOTDIR", dir);
        }

        let child = pair.slave.spawn_command(cmd).map_err(|e| format!("failed to start {shell}: {e}"))?;
        drop(pair.slave);
        let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
        let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;

        std::thread::spawn(move || {
            let mut carry = Utf8Carry::default();
            let mut buf = [0u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        let text = carry.push(&buf[..n]);
                        if !text.is_empty() {
                            on_output(text);
                        }
                    }
                }
            }
            on_exit();
        });

        self.sessions.insert(id, Session { master: pair.master, child, writer, zdot });
        Ok(id)
    }

    pub fn write(&mut self, id: u32, data: &str) -> Result<(), String> {
        let session = self.sessions.get_mut(&id).ok_or("terminal not found")?;
        session.writer.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
        session.writer.flush().map_err(|e| e.to_string())
    }

    pub fn resize(&mut self, id: u32, cols: u16, rows: u16) -> Result<(), String> {
        let session = self.sessions.get(&id).ok_or("terminal not found")?;
        session.master.resize(size(cols, rows)).map_err(|e| e.to_string())
    }

    pub fn close(&mut self, id: u32) {
        if let Some(mut session) = self.sessions.remove(&id) {
            kill_tree(&mut session);
        }
    }

    pub fn close_all(&mut self) {
        let ids: Vec<u32> = self.sessions.keys().copied().collect();
        for id in ids {
            self.close(id);
        }
    }
}

fn kill_tree(session: &mut Session) {
    #[cfg(unix)]
    if let Some(pid) = session.child.process_id() {
        unsafe extern "C" {
            fn killpg(pgrp: i32, sig: i32) -> i32;
        }
        unsafe {
            killpg(pid as i32, 1);
        }
    }
    let _ = session.child.kill();
    let _ = session.child.wait();
    if let Some(dir) = session.zdot.take() {
        let _ = std::fs::remove_dir_all(dir);
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn runs_a_shell_and_streams_output() {
        let dir = tempfile::tempdir().unwrap();
        let (tx, rx) = mpsc::channel::<String>();
        let (done_tx, done_rx) = mpsc::channel::<()>();
        let mut manager = TerminalManager::default();
        let id = manager
            .open(Some("/bin/sh".into()), dir.path(), 80, 24, move |text| { let _ = tx.send(text); }, move || {
                let _ = done_tx.send(());
            })
            .unwrap();
        manager.write(id, "echo stynx-$((40+2))\nexit\n").unwrap();
        done_rx.recv_timeout(Duration::from_secs(10)).unwrap();
        let output: String = rx.try_iter().collect();
        assert!(output.contains("stynx-42"), "output was {output:?}");
        manager.close(id);
    }
}
