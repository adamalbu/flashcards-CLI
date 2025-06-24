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
    use ratatui::crossterm::event;
    use ratatui::crossterm::event::{Event, KeyCode};
    use ratatui::prelude::*;
    use ratatui::symbols::border;
    use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
    use ratatui::{DefaultTerminal, Frame};
    use std::io;

    pub enum Page {
        SetList,
        CreateSet,
    }

    pub struct App {
        pub sets: Vec<flashcards::Set>,
        pub page: Page,
        pub set_name_input: String,
        exit: bool,
    }

    impl App {
        pub fn new() -> Self {
            Self {
                sets: Vec::new(),
                page: Page::SetList,
                set_name_input: String::new(),
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
            match (&self.page, key_event.code) {
                (Page::SetList, KeyCode::Char('n')) => {
                    self.page = Page::CreateSet;
                    self.set_name_input.clear();
                },

                (Page::CreateSet, KeyCode::Esc) => self.page = Page::SetList,
                (Page::CreateSet, KeyCode::Enter) => {self.create_set(); self.page = Page::SetList},
                (Page::CreateSet, KeyCode::Char(c)) => {
                    self.set_name_input.push(c);
                },
                (Page::CreateSet, KeyCode::Backspace) => {
                    self.set_name_input.pop();
                },
                
                (_, KeyCode::Char('q')) | (_, KeyCode::Esc) => self.exit(),
                _ => {}
            }
        }

        fn create_set(&mut self) {
            let set = flashcards::Set::new(self.set_name_input.clone());
            self.sets.push(set);
        }

        fn exit(&mut self) {
            todo!();
        }
    }

    impl App {
        fn render_set_list(&self, area: Rect, buf: &mut Buffer) {
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
                .map(|set| Line::from(set.name.as_str()).on_dark_gray().black())
                .collect();
            let list =
                List::new(sets.into_iter().map(ListItem::new).collect::<Vec<_>>()).block(block);

            Widget::render(list, area, buf);
        }

        fn render_create_set(&self, area: Rect, buf: &mut Buffer) {
            let title = Line::from(" Create Set ");
            let block = Block::bordered().title(title).border_set(border::DOUBLE);
            block.render(area, buf);

            let prompt_height = 3;
            let prompt_y = area.y + (area.height.saturating_sub(prompt_height)) / 2;
            let prompt_area = Rect::new(
                area.x + 2,
                prompt_y,
                area.width.saturating_sub(4),
                prompt_height,
            );

            let input = Paragraph::new(self.set_name_input.as_str())
                .block(Block::default().title("Set Name").on_dark_gray().black().borders(Borders::ALL));
            Widget::render(input, prompt_area, buf);
        }
    }

    impl Widget for &App {
        fn render(self, area: Rect, buf: &mut Buffer) {
            match self.page {
                Page::SetList => self.render_set_list(area, buf),
                Page::CreateSet => self.render_create_set(area, buf),
            }
        }
    }
}
