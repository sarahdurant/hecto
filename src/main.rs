mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Cursor;


// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     byte &0b0001_1111
// }

fn main()  {
    let _ = Editor::default().run();
}