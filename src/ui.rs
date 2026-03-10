use crossterm::{
    cursor,
    execute,
    style::{self, Color, Stylize},
    terminal,
};
use std::io::{self, Write};

use crate::task::{Status, Task};

const BANNER: &str = r#"
  ██████╗  █████╗ ██╗██╗  ██╗   ██╗     ██████╗██╗  ██╗███████╗ ██████╗██╗  ██╗
  ██╔══██╗██╔══██╗██║██║  ╚██╗ ██╔╝    ██╔════╝██║  ██║██╔════╝██╔════╝██║ ██╔╝
  ██║  ██║███████║██║██║   ╚████╔╝     ██║     ███████║█████╗  ██║     █████╔╝
  ██║  ██║██╔══██║██║██║    ╚██╔╝      ██║     ██╔══██║██╔══╝  ██║     ██╔═██╗
  ██████╔╝██║  ██║██║███████╗██║       ╚██████╗██║  ██║███████╗╚██████╗██║  ██╗
  ╚═════╝ ╚═╝  ╚═╝╚═╝╚══════╝╚═╝        ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝
"#;

const HELP_ENTRIES: &[(&str, &str)] = &[
    ("↑↓ jk", "Naviguer"),
    ("espace", "Changer état"),
    ("a", "Ajouter"),
    ("d", "Supprimer"),
    ("e", "Éditer"),
    ("h", "Historique"),
    ("s", "Sauvegarder"),
    ("q", "Quitter"),
];

pub enum InputMode<'a> {
    Add(&'a str),
    Edit(usize, &'a str),
}

pub fn clear(stdout: &mut impl Write) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
}

pub fn draw(
    stdout: &mut impl Write,
    tasks: &[Task],
    selected: usize,
    date: &str,
    input_mode: Option<&InputMode>,
) -> io::Result<()> {
    clear(stdout)?;

    for line in BANNER.lines() {
        execute!(
            stdout,
            style::PrintStyledContent(line.with(Color::Rgb { r: 80, g: 160, b: 255 })),
            style::Print("\r\n"),
        )?;
    }

    execute!(
        stdout,
        style::Print("\r\n"),
        style::Print("  "),
        style::PrintStyledContent("# Développé par  ".with(Color::Rgb { r: 80, g: 160, b: 255 })),
        style::PrintStyledContent("Dawson PRIANON".bold().with(Color::Rgb { r: 80, g: 160, b: 255 })),
        style::Print("\r\n"),
        style::Print("\r\n"),
    )?;

    execute!(
        stdout,
        style::PrintStyledContent(format!("  Checklist du {}\r\n", date).with(Color::Grey)),
    )?;

    let sep = "  ".to_string() + &"─".repeat(72);
    execute!(
        stdout,
        style::PrintStyledContent(sep.with(Color::DarkGrey)),
        style::Print("\r\n"),
    )?;

    if tasks.is_empty() && !matches!(input_mode, Some(InputMode::Add(_))) {
        execute!(
            stdout,
            style::PrintStyledContent(
                "  Aucune tâche. Appuie sur 'a' pour en ajouter une.\r\n".with(Color::Grey)
            ),
        )?;
    } else {
        for (i, task) in tasks.iter().enumerate() {
            let title_override = match input_mode {
                Some(InputMode::Edit(idx, buf)) if *idx == i => Some(*buf),
                _ => None,
            };
            draw_task(stdout, task, i == selected, title_override)?;
        }

        if let Some(InputMode::Add(buf)) = input_mode {
            draw_input_row(stdout, buf)?;
        }
    }

    execute!(stdout, style::Print("\r\n"))?;
    let sep = "  ".to_string() + &"─".repeat(72);
    execute!(
        stdout,
        style::PrintStyledContent(sep.with(Color::DarkGrey)),
        style::Print("\r\n"),
    )?;

    execute!(stdout, style::Print("  "))?;
    for (i, (key, desc)) in HELP_ENTRIES.iter().enumerate() {
        if i > 0 {
            execute!(stdout, style::PrintStyledContent("  │  ".with(Color::DarkGrey)))?;
        }
        execute!(
            stdout,
            style::PrintStyledContent("[".with(Color::DarkGrey)),
            style::PrintStyledContent(key.bold().with(Color::White)),
            style::PrintStyledContent("]".with(Color::DarkGrey)),
            style::Print(" "),
            style::PrintStyledContent(desc.with(Color::Grey)),
        )?;
    }
    execute!(stdout, style::Print("\r\n"))?;

    stdout.flush()
}

fn draw_task(
    stdout: &mut impl Write,
    task: &Task,
    selected: bool,
    title_override: Option<&str>,
) -> io::Result<()> {
    let (status_color, status_label) = match task.status {
        Status::Todo  => (Color::Rgb { r: 255, g: 200, b:  40 }, task.status.label()),
        Status::Doing => (Color::Rgb { r:  80, g: 160, b: 255 }, task.status.label()),
        Status::Done  => (Color::Rgb { r:  80, g: 210, b:  80 }, task.status.label()),
    };

    let displayed_title: String = match title_override {
        Some(buf) => format!("{}▌", buf),
        None => task.title.clone(),
    };

    if selected {
        execute!(
            stdout,
            style::PrintStyledContent(" ▶ ".with(Color::White)),
            style::PrintStyledContent(status_label.with(status_color).bold()),
            style::Print(" "),
            style::PrintStyledContent(displayed_title.on(Color::DarkGrey).with(status_color).bold()),
            style::Print("\r\n"),
        )
    } else {
        let title_color = match task.status {
            Status::Done => Color::Grey,
            _ => status_color,
        };
        execute!(
            stdout,
            style::PrintStyledContent("   ".with(Color::DarkGrey)),
            style::PrintStyledContent(status_label.with(status_color)),
            style::Print(" "),
            style::PrintStyledContent(displayed_title.with(title_color)),
            style::Print("\r\n"),
        )
    }
}

/// Affiche le sélecteur d'historique. `files` est une liste de dates au format DD-MM-YYYY.
pub fn draw_history(stdout: &mut impl Write, files: &[String], selected: usize) -> io::Result<()> {
    clear(stdout)?;

    for line in BANNER.lines() {
        execute!(
            stdout,
            style::PrintStyledContent(line.with(Color::Rgb { r: 80, g: 160, b: 255 })),
            style::Print("\r\n"),
        )?;
    }

    execute!(
        stdout,
        style::Print("\r\n"),
        style::Print("  "),
        style::PrintStyledContent("# Développé par  ".with(Color::Rgb { r: 80, g: 160, b: 255 })),
        style::PrintStyledContent("Dawson PRIANON".bold().with(Color::Rgb { r: 80, g: 160, b: 255 })),
        style::Print("\r\n"),
        style::Print("\r\n"),
    )?;

    execute!(
        stdout,
        style::PrintStyledContent("  Historique\r\n".with(Color::Grey)),
    )?;

    let sep = "  ".to_string() + &"─".repeat(72);
    execute!(
        stdout,
        style::PrintStyledContent(sep.clone().with(Color::DarkGrey)),
        style::Print("\r\n"),
    )?;

    if files.is_empty() {
        execute!(
            stdout,
            style::PrintStyledContent("  Aucun fichier trouvé.\r\n".with(Color::Grey)),
        )?;
    } else {
        for (i, date) in files.iter().enumerate() {
            if i == selected {
                execute!(
                    stdout,
                    style::PrintStyledContent(" ▶ ".with(Color::White)),
                    style::PrintStyledContent(date.clone().on(Color::DarkGrey).with(Color::White).bold()),
                    style::Print("\r\n"),
                )?;
            } else {
                execute!(
                    stdout,
                    style::PrintStyledContent("   ".with(Color::DarkGrey)),
                    style::PrintStyledContent(date.clone().with(Color::Grey)),
                    style::Print("\r\n"),
                )?;
            }
        }
    }

    execute!(stdout, style::Print("\r\n"))?;
    execute!(
        stdout,
        style::PrintStyledContent(sep.with(Color::DarkGrey)),
        style::Print("\r\n"),
    )?;

    execute!(
        stdout,
        style::Print("  "),
        style::PrintStyledContent("[".with(Color::DarkGrey)),
        style::PrintStyledContent("↑↓ jk".bold().with(Color::White)),
        style::PrintStyledContent("]".with(Color::DarkGrey)),
        style::Print(" "),
        style::PrintStyledContent("Naviguer".with(Color::Grey)),
        style::PrintStyledContent("  │  ".with(Color::DarkGrey)),
        style::PrintStyledContent("[".with(Color::DarkGrey)),
        style::PrintStyledContent("Entrée".bold().with(Color::White)),
        style::PrintStyledContent("]".with(Color::DarkGrey)),
        style::Print(" "),
        style::PrintStyledContent("Ouvrir".with(Color::Grey)),
        style::PrintStyledContent("  │  ".with(Color::DarkGrey)),
        style::PrintStyledContent("[".with(Color::DarkGrey)),
        style::PrintStyledContent("Échap".bold().with(Color::White)),
        style::PrintStyledContent("]".with(Color::DarkGrey)),
        style::Print(" "),
        style::PrintStyledContent("Annuler".with(Color::Grey)),
        style::Print("\r\n"),
    )?;

    stdout.flush()
}

fn draw_input_row(stdout: &mut impl Write, buf: &str) -> io::Result<()> {
    let todo_color = Color::Rgb { r: 255, g: 200, b: 40 };
    execute!(
        stdout,
        style::PrintStyledContent(" ▶ ".with(Color::White)),
        style::PrintStyledContent(Status::Todo.label().with(todo_color).bold()),
        style::Print(" "),
        style::PrintStyledContent(format!("{}▌", buf).on(Color::DarkGrey).with(Color::White)),
        style::Print("\r\n"),
    )
}
