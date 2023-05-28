use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Corner, Rect};
use tui::style::{Style, Color};
use tui::widgets::{ListState, Block, Borders, List, ListItem};
use super::Drawable;

pub struct SelectableList<T> {
    list : Vec<T>,
    title: Option<String>,
    index: Option<usize>,
}

impl<T> SelectableList<T> {
    pub fn new(list: Vec<T>) -> Self {
        Self { list, title: None, index: None }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn next(mut self, total_size: usize) -> Self {
        self.index = self.index
            .map(|e| (e + 1).rem_euclid(total_size))
            .or(Some(0));
        self
    }

    pub fn prev(mut self, total_size: usize) -> Self {
        self.index = self.index
            .map(|e| (e as i32 - 1).rem_euclid(total_size as i32) as usize)
            .or(Some(0));
        self
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn make_drawable<F>(&self, item_builder: F) -> DrawableList<T, F>
    where F: Fn(&T, Rect) -> String,
    {
        DrawableList { list: self, item_builder }
    }
}

pub struct DrawableList<'a, T, F>
where F: Fn(&T, Rect) -> String,
{
    list: &'a SelectableList<T>,
    item_builder: F,
}

impl<'a, T, F> Drawable for DrawableList<'a, T, F>
where F: Fn(&T, Rect) -> String,
{
    fn draw<B: Backend>(&self, f: &mut Frame<B>)
    where F: Fn(&T, Rect) -> String,
    {
        let mut state = ListState::default();
        state.select(self.list.index);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.list.title.clone().unwrap_or("".to_string()));

        let list = List::new(
            &self.list.list
                .iter()
                .map(|i| (self.item_builder)(i, f.size()))
                .map(|i| ListItem::new(i))
                .collect::<Vec<ListItem<'_>>>()[..]
            )
            .block(block)
            .start_corner(Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
             );

        f.render_stateful_widget(list, f.size(), &mut state)
    }
}

