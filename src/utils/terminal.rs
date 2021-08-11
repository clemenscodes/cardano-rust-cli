use console::Color;
use console::Emoji;
use console::Style;
use console::Term;
use std::process::Stdio;
use tokio::process::Command;

pub struct Terminal;

impl Terminal {
    pub async fn async_command(color: &str, command: &str, emoji: Emoji<'_, '_>) {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .await;
        if let Ok(output) = output {
            let stdout = &String::from_utf8_lossy(&output.stdout);
            Terminal::print(color, stdout, emoji);
        } else {
            panic!("Error executing command");
        }
    }

    pub fn print(color: &str, output: &str, emoji: Emoji) {
        let color: Color = Terminal::to_color(&color);
        match color {
            Color::Cyan => {
                let cyan = format!("{} {}", Style::new().cyan().apply_to(output), emoji);
                Terminal::write(&cyan)
            }
            Color::Blue => {
                let blue = format!("{} {}", Style::new().blue().apply_to(output), emoji);
                Terminal::write(&blue)
            }
            Color::Black => {
                let black = format!("{} {}", Style::new().black().apply_to(output), emoji);
                Terminal::write(&black)
            }
            Color::Red => {
                let red = format!("{} {}", Style::new().red().apply_to(output), emoji);
                Terminal::write(&red)
            }
            Color::Green => {
                let green = format!("{} {}", Style::new().green().apply_to(output), emoji);
                Terminal::write(&green)
            }
            Color::Yellow => {
                let yellow = format!("{} {}", Style::new().yellow().apply_to(output), emoji);
                Terminal::write(&yellow)
            }
            Color::Magenta => {
                let magenta = format!("{} {}", Style::new().magenta().apply_to(output), emoji);
                Terminal::write(&magenta)
            }
            Color::White => {
                let white = format!("{} {}", Style::new().white().apply_to(output), emoji);
                Terminal::write(&white)
            }
            _ => {
                let white = format!("{} {}", Style::new().white().apply_to(output), emoji);
                Terminal::write(&white)
            }
        };
    }

    fn to_color(color: &str) -> Color {
        match color {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            _ => Color::White,
        }
    }

    fn write(output: &str) {
        Term::stdout().write_line(output).expect("Failed printing to console")
    }
}