mod error;
mod terminal;

pub use terminal::Crossterminal;
pub use error::Error;
use tui::{
    widgets::{
        Block,
        Borders,
        List,
        ListItem,
    },
    Frame,
    backend::Backend,
    style::{Style, Color},
};

pub struct FullListView<'a> {
    record_list: Vec<ListItem<'a>>,
}

impl<'a> FullListView<'a> {
    pub fn new(items: impl Into<Vec<ListItem<'a>>>) -> Self {
        Self {
            record_list: items.into(),
        }
    }

    pub fn draw_app<B>(&mut self, f: &mut Frame<B>)
    where B: Backend {
        let size = f.size();
        let items = List::new(&self.record_list[..])
            .block(Block::default().borders(Borders::ALL).title("Entries"))
            .start_corner(tui::layout::Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
             );
        f.render_widget(items, size);
    }
}
