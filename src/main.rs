#![warn(clippy::all, clippy::pedantic)]
mod editor;

use crate::editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
