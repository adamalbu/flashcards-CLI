use ratatui::Frame;
use std::io;

mod flashcards {
    pub struct Set {
        pub name: String,
        pub cards: Vec<Card>,
    }

    impl Set {
        pub fn new(name: String) -> Self {
            Self {
                name,
                cards: Vec::new(),
            }
        }

        pub fn add_card(&mut self, front: String, back: String) {
            self.cards.push(Card { front, back });
        }
    }

    pub struct Card {
        pub front: String,
        pub back: String,
    }
}

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let mut app = app::App::new();
    app.sets.push(flashcards::Set::new(String::from("Test")));
    app.sets.push(flashcards::Set::new(String::from("Test2")));
    let result = app.run(terminal);
    ratatui::restore();
    result
}

mod app {
    use crate::flashcards;
    use crossterm::event;
    use crossterm::event::{Event, KeyCode};
    use ratatui::prelude::*;
    use ratatui::symbols::border;
    use ratatui::widgets::{Block, List, ListItem};
    use ratatui::{DefaultTerminal, Frame};
    use std::io;

    pub struct App {
        pub sets: Vec<flashcards::Set>,
        exit: bool,
    }

    impl App {
        pub fn new() -> Self {
            Self {
                sets: Vec::new(),
                exit: false,
            }
        }

        pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
            while !self.exit {
                terminal.draw(|frame| self.draw(frame))?;
                self.handle_events()?
            }
            Ok(())
        }

        fn draw(&self, frame: &mut Frame) {
            frame.render_widget(self, frame.area());
        }

        fn handle_events(&mut self) -> io::Result<()> {
            match event::read()? {
                Event::Key(key_event) => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            }
            Ok(())
        }

        fn handle_key_event(&mut self, key_event: event::KeyEvent) {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                KeyCode::Char('n') => self.create_set(),
                _ => {}
            }
        }

        fn create_set(&mut self) {
            todo!();
        }

        fn exit(&mut self) {
            todo!();
        }
    }

    impl Widget for &App {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let title = Line::from(" Sets ");
            let commands = Line::from(vec![
                " New set ".into(),
                "[N] ".blue().bold(),
                "Quit ".into(),
                "[Q] ".red().bold(),
            ]);

            let block = Block::bordered()
                .title(title)
                .title_bottom(commands)
                .border_set(border::DOUBLE);
            let sets: Vec<Line> = self
                .sets
                .iter()
                .map(|set| Line::from(set.name.as_str()).on_dark_gray())
                .collect();
            let list =
                List::new(sets.into_iter().map(ListItem::new).collect::<Vec<_>>()).block(block);
            // block.render(area, buf);
            Widget::render(list, area, buf);
        }
    }
}
fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
