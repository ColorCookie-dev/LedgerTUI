use tui::{
    widgets::{
        Block,
        Borders,
        List,
        ListItem,
        ListState,
    },
    Frame,
    backend::Backend,
    style::{Style, Color},
};

pub struct RecordList<'a> {
    record_list: Vec<ListItem<'a>>,
    state: ListState,
}

impl<'a> RecordList<'a> {
    pub fn new(items: impl Into<Vec<ListItem<'a>>>) -> Self {
        Self {
            record_list: items.into(),
            state: ListState::default(),
        }
    }

    pub fn draw_app<B>(&mut self, f: &mut Frame<B>)
    where B: Backend {
        let size = f.size();
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Entries");

        let items = List::new(&self.record_list[..])
            .block(block)
            .start_corner(tui::layout::Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
             );

        f.render_widget(items, size);
    }
}
