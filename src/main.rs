#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
pub use editor::Position;

use crate::editor::Editor;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
