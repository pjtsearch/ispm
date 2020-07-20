use std::io;
use std::io::Write;

pub fn confirm(message:&str,default:bool) -> io::Result<bool> {
    let default_str = match default {
        true => "[Y/n]",
        false => "[y/N]"
    };
    print!("{} {}:",message,default_str);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input == "Y\n" || input == "y\n" || (default && input == "\n"))
}