use std::{
    //collections::HashMap,
    io::{self},
    time::{Duration},
};
use crossterm::{
    event::{poll, read, Event, KeyEvent, KeyModifiers, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

// fn fatal_err(e: std::io::Error) {
//     panic!("{}", e);
// }

// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     byte &0b0001_1111
// }

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    loop {
        if poll(Duration::from_millis(500))?{
            match read()? {
                Event::Key(event) => {

                    let key = event.code;
                    let modifiers = event.modifiers;
                    
                    println!("{:?}\r", key);         
                    println!("{:?}\r", modifiers);         
                    if key == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
                        println!("Exiting... Good-bye!");
                        break;
                    }
                },
                _ => (),
            }
        }
    }
    // for key in io::stdin().keys() {
    //     match key {
    //         Ok(key) => match key {

    //             Key::Char(c) => {
    //                 if c.is_control() {
    //                     println!("{:?} \r", b);
    //                 } else {
    //                     println!("{:?} ({})\r", b, c);
    //                 }
    //                 if b == to_ctrl_byte('q') {
    //                     break;
    //                 }
    //             }
    //             Key::Ctrl('q') => break,
    //             _ => println!("{:?}\r", key),
    //         },
    //         Err(e) => fatal_err(e),
    //     }
    // }
    disable_raw_mode()?;
    Ok(())
}