use crate::utils::cmd::TextType::*;
use std::io;
use std::io::Write;
use std::process::Command;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::constants::{MAIN_COLOR, NAME, SECONDARY_COLOR, VERSION};

fn c(t: (u8, u8, u8)) -> termcolor::Color {
    termcolor::Color::Rgb(t.0, t.1, t.2)
}

pub fn clear_console() {
    Command::new("cmd.exe")
        .arg("/c")
        .arg("cls")
        .status()
        .expect("cannot clear");
}

pub fn pause() {
    Text::new(
        LN("   Press any key to continue...".to_string()),
        Color::Rgb(128, 128, 128),
    )
    .w();
    let _ = Command::new("cmd.exe").arg("/c").arg("pause >nul").status();
}

pub enum TextType {
    L(String),
    LN(String),
}

pub struct Text {
    pub ttype: TextType,
    pub color: Color,
}

impl Text {
    pub fn new(ttype: TextType, color: Color) -> Self {
        Text { ttype, color }
    }

    pub fn w(self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout
            .set_color(ColorSpec::new().set_fg(Some(self.color)))
            .expect("Unable to set color");
        match self.ttype {
            L(text) => {
                write!(&mut stdout, "{}", text).expect("Unable to write");
            }
            LN(text) => {
                writeln!(&mut stdout, "{}", text).expect("Unable to write");
            }
        }
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .expect("Unable to set color");
    }
}

pub enum Input {
    String(String),
    None,
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub items: Vec<String>,
    pub title: String,
}

impl Menu {
    pub fn new(title: &str, items: Vec<String>) -> Self {
        winconsole::console::set_title(&format!("{} {} | {}", NAME, VERSION, title))
            .expect("Unable to set title");
        Menu {
            items,
            title: title.to_string(),
        }
    }

    pub fn show(self) -> usize {
        loop {
            Menu::title();

            Text::new(L("  [".to_string()), c(SECONDARY_COLOR)).w();
            Text::new(L(self.title.clone()), c(MAIN_COLOR)).w();
            Text::new(LN("]".to_string()), c(SECONDARY_COLOR)).w();
            println!();

            for (index, item) in self.items.iter().enumerate() {
                Text::new(L("  (".to_string()), c(SECONDARY_COLOR)).w();
                let num = format!("{}", index + 1);
                Text::new(L(num), c(MAIN_COLOR)).w();
                Text::new(L(") ".to_string()), c(SECONDARY_COLOR)).w();
                Text::new(LN(item.to_string()), c(MAIN_COLOR)).w();
            }
            let input = Menu::input(Input::None);
            match input.parse::<usize>() {
                Ok(num) => {
                    return num;
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    pub fn input(inp: Input) -> String {
        match inp {
            Input::String(s) => {
                Text::new(L(format!("   {}", s)), c(MAIN_COLOR)).w();
                Text::new(L(" > ".to_string()), c(SECONDARY_COLOR)).w();
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input.trim().to_string()
            }
            Input::None => {
                Text::new(L("   > ".to_string()), c(SECONDARY_COLOR)).w();
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input.trim().to_string()
            }
        }
    }

    fn lerp_color(
        color1: (u8, u8, u8),
        color2: (u8, u8, u8),
        step: usize,
        total_steps: usize,
    ) -> (u8, u8, u8) {
        let t = step as f32 / (total_steps - 1) as f32;
        let (r1, g1, b1) = color1;
        let (r2, g2, b2) = color2;

        let r = ((1.0 - t) * r1 as f32 + t * r2 as f32) as u8;
        let g = ((1.0 - t) * g1 as f32 + t * g2 as f32) as u8;
        let b = ((1.0 - t) * b1 as f32 + t * b2 as f32) as u8;

        (r, g, b)
    }

    pub fn title() {
        let (width, _) = crossterm::terminal::size().expect("cannot get");
        clear_console();
        println!();
        let title = vec![
            r#"   U  ___ u  ____   U _____ u   ____        _       ____  __  __   "#,
            r#"    \/"_ \/U|  _"\ u\| ___"|/U |  _"\ u U  /"\  uU /"___|u\ \/"/   "#,
            r#"    | | | |\| |_) |/ |  _|"   \| |_) |/  \/ _ \/ \| |  _ //\  /\   "#,
            r#".-,_| |_| | |  __/   | |___    |  _ <    / ___ \  | |_| |U /  \ u  "#,
            r#" \_)-\___/  |_|      |_____|   |_| \_\  /_/   \_\  \____| /_/\_\   "#,
            r#"      \\    ||>>_    <<   >>   //   \\_  \\    >>  _)(|_,-,>> \\_  "#,
            r#"     (__)  (__)__)  (__) (__) (__)  (__)(__)  (__)(__)__)\_)  (__) "#,
        ];

        let num_lines = title.len();

        for (line_num, line) in title.iter().enumerate() {
            let x = Menu::center(line, width as usize);
            let color = Menu::lerp_color(MAIN_COLOR, SECONDARY_COLOR, line_num, num_lines);
            //dbg!(color);
            Text::new(LN(x), c(color)).w();
        }

        println!();
    }

    pub fn center(s: &str, screen_width: usize) -> String {
        let padding = ((screen_width) - s.len()) / 2;
        let left_padding = " ".repeat(padding);
        format!("{}{}", left_padding, s)
    }
}
