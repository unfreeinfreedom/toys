// this is a toy screensaver i made because of some script inspired from r/unixporn
// procedure:
// TODO: - [X] first check is the terminal a tty
// TODO: - [ ] now load crossterm

use std::io::stdout;
use crossterm::tty::IsTty;

fn main(){
    if (stdout().is_tty()) == true {
        println!("yes, its a tty");
    } else {
        println!("sorry, a tty is needed to show this screensaver");
    }
}
