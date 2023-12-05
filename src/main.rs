mod editor;

use editor::Editor;

// fn fatal_err(e: std::io::Error) {
//     panic!("{}", e);
// }

// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     byte &0b0001_1111
// }

fn main()  {
    let editor = Editor::default();
    let _ = editor.run();
}