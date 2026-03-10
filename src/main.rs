mod task;
mod storage;
mod ui;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal,
};
use std::io::{self, stdout};
use std::path::PathBuf;

use task::Task;
use ui::InputMode;

/// Convertit un chemin `YYYY-MM-DD.txt` en label `DD-MM-YYYY`.
fn date_label(path: &PathBuf) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| {
            let p: Vec<&str> = s.split('-').collect();
            if p.len() == 3 { format!("{}-{}-{}", p[2], p[1], p[0]) } else { s.to_string() }
        })
        .unwrap_or_default()
}

fn main() -> io::Result<()> {
    storage::ensure_data_dir()?;

    let mut current_path = storage::today_file();
    let mut tasks: Vec<Task> = storage::load_tasks(&current_path)?;
    let mut selected: usize = 0;

    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;

    loop {
        if !tasks.is_empty() && selected >= tasks.len() {
            selected = tasks.len() - 1;
        }

        ui::draw(&mut stdout, &tasks, selected, &date_label(&current_path), None)?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match (key.code, key.modifiers) {
                (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,

                (KeyCode::Up, _) | (KeyCode::Char('k'), _) => {
                    if !tasks.is_empty() && selected > 0 {
                        selected -= 1;
                    }
                }
                (KeyCode::Down, _) | (KeyCode::Char('j'), _) => {
                    if !tasks.is_empty() && selected + 1 < tasks.len() {
                        selected += 1;
                    }
                }

                (KeyCode::Char(' '), _) => {
                    if let Some(task) = tasks.get_mut(selected) {
                        task.status = task.status.next();
                        storage::save_tasks(&current_path, &tasks)?;
                    }
                }

                (KeyCode::Char('a'), _) => {
                    let mut buf = String::new();
                    loop {
                        ui::draw(&mut stdout, &tasks, selected, &date_label(&current_path), Some(&InputMode::Add(&buf)))?;
                        if let Event::Key(key) = event::read()? {
                            if key.kind != KeyEventKind::Press { continue; }
                            match key.code {
                                KeyCode::Enter => {
                                    let title = buf.trim().to_string();
                                    if !title.is_empty() {
                                        tasks.push(Task::new(title));
                                        selected = tasks.len() - 1;
                                        storage::save_tasks(&current_path, &tasks)?;
                                    }
                                    break;
                                }
                                KeyCode::Esc => break,
                                KeyCode::Backspace => { buf.pop(); }
                                KeyCode::Char(c) => buf.push(c),
                                _ => {}
                            }
                        }
                    }
                }

                (KeyCode::Char('d'), _) => {
                    if !tasks.is_empty() {
                        tasks.remove(selected);
                        if selected > 0 && selected >= tasks.len() {
                            selected -= 1;
                        }
                        storage::save_tasks(&current_path, &tasks)?;
                    }
                }

                (KeyCode::Char('e'), _) => {
                    if !tasks.is_empty() {
                        let mut buf = tasks[selected].title.clone();
                        loop {
                            ui::draw(&mut stdout, &tasks, selected, &date_label(&current_path), Some(&InputMode::Edit(selected, &buf)))?;
                            if let Event::Key(key) = event::read()? {
                                if key.kind != KeyEventKind::Press { continue; }
                                match key.code {
                                    KeyCode::Enter => {
                                        let title = buf.trim().to_string();
                                        if !title.is_empty() {
                                            tasks[selected].title = title;
                                            storage::save_tasks(&current_path, &tasks)?;
                                        }
                                        break;
                                    }
                                    KeyCode::Esc => break,
                                    KeyCode::Backspace => { buf.pop(); }
                                    KeyCode::Char(c) => buf.push(c),
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                (KeyCode::Char('s'), _) => {
                    storage::save_tasks(&current_path, &tasks)?;
                }

                (KeyCode::Char('h'), _) => {
                    let files = storage::list_files()?;
                    let labels: Vec<String> = files.iter().map(date_label).collect();
                    let mut hist_sel: usize = 0;

                    loop {
                        ui::draw_history(&mut stdout, &labels, hist_sel)?;
                        if let Event::Key(key) = event::read()? {
                            if key.kind != KeyEventKind::Press { continue; }
                            match key.code {
                                KeyCode::Up | KeyCode::Char('k') => {
                                    if hist_sel > 0 { hist_sel -= 1; }
                                }
                                KeyCode::Down | KeyCode::Char('j') => {
                                    if hist_sel + 1 < files.len() { hist_sel += 1; }
                                }
                                KeyCode::Enter | KeyCode::Char('l') => {
                                    current_path = files[hist_sel].clone();
                                    tasks = storage::load_tasks(&current_path)?;
                                    selected = 0;
                                    break;
                                }
                                KeyCode::Esc => break,
                                _ => {}
                            }
                        }
                    }
                }

                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;

    Ok(())
}
