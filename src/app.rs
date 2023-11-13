pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
}

use std::sync::Mutex;

use lazy_static::lazy_static;
use ratatui::{text, widgets::ScrollbarState};

lazy_static! {
    pub static ref MESSAGES: Mutex<Vec<Vec<text::Line<'static>>>> = Mutex::new(Vec::new());
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Events", "Placeholder", "Placeholder"]),
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
        }
    }
    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn scroll_up(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    pub fn scroll_down(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'j' => self.scroll_up(),
            'k' => self.scroll_down(),
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}
