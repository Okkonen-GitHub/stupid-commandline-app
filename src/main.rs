extern crate clap;
extern crate clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};
use clap::{App};

fn main() {
    let matches = App::new("Decorator")
                            .version("1.0")
                            .author("Okko Oikkonen <okko.oikkonen@gmail.com>")
                            .about("Decorates a message and copies it to the clipboard")
                            .args_from_usage("
                            -m --mode=[1-2] 'decoration style modifier'
                            <INPUT> 'Message to decorate. Use \"\" for multiple words'
                            ")
                            
                            .get_matches();
    let mode;
    if matches.is_present("mode") {
        mode = matches.values_of("mode").unwrap().nth(0).unwrap();
    } else {
        mode = "2";
    }
    let input = matches.values_of("INPUT").unwrap().nth(0).unwrap();
    println!("Preview:\n{}", decorate(input, mode.parse::<i32>().unwrap()));
    let succces = copy_to_cb(decorate(input, mode.parse::<i32>().unwrap()));
    println!("\nCopied to clipboard: {}", succces);
}

fn decorate(text: &str, mode: i32) -> String {
    let mut result: String = String::new();
    if mode == 1 {
        result += "𝗜𝗺𝗽𝗼𝗿𝘁𝗮𝗻𝘁​​​​​\n";
        for _thing in 0..text.chars().count()+4 {
            result += "■";
        }
        
        result += "\n";
        result += "■ ";
        result += text;
        result += " ■";
        result += "\n";
        for _thing in 0..text.chars().count()+4 {
            result += "■";
        }
    }
    else if mode == 2 {
        result += "𝖫𝗈𝗈𝗄 𝖺𝗍 𝗍𝗁𝗂𝗌\n";
        for _thing in 0..text.chars().count()+4 {
            result += "▢";
        }
        result += "\n";
        result += "▢ ";
        result += text;
        result += " ▢";
        result += "\n";
        for _thing in 0..text.chars().count()+4 {
            result += "▢";
        }
    }
    return result;
    
}

fn copy_to_cb(copy_text: String) -> &'static str {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(copy_text.to_owned()).unwrap();
    return "Success"
}
