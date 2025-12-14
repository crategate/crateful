use std::path::PathBuf;

use ratatui::{
    layout::Rect,
    prelude::Buffer,
    widgets::{StatefulWidget, StatefulWidgetRef},
};

pub struct PathStates {
    save_a: PathBuf,
    save_d: PathBuf,
    save_g: PathBuf,
}
pub struct Instructs {
    state: PathStates,
}
impl Instructs {}

impl StatefulWidget for Instructs {
    type State: PathStates;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidgetRef::render_ref(&self, area, buf, state);
    }
}
