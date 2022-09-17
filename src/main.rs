use std::{thread,time::Duration,process::Command};
use colorgrad::{Gradient};

fn map(colors:&Gradient)->Vec<String>{
    let mut l = vec![];
    for col in colors.colors(50){
        l.push(col.to_hex_string())
    }
    l
}

fn border(hex: &String){
    Command::new("hyprctl")
        .arg("keyword")
        .arg("dwindle:col.group_border_active")
        .arg(format!("0xff{}",hex.replace("#", "")))
        .spawn()
        .expect("Failed to execute process active border");
    Command::new("hyprctl")
        .arg("keyword")
        .arg("dwindle:col.group_border")
        .arg(format!("0x88{}",hex.replace("#", "")))
        .spawn()
        .expect("Failed to execute process");
}

fn main() {
   let grad = colorgrad::rainbow();
   let freq = Duration::from_millis(80);
   loop{
    for col in map(&grad){
        border(&col);
        thread::sleep(freq);
    }
   }
}
