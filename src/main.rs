extern crate reqwest;
extern crate argparse;
use colorized::*;
use std::process::Command;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use argparse::{ArgumentParser, StoreTrue, Store};

use reqwest::Client;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    banner();

    let mut port = String::from("8060");
    let mut target = String::from("127.0.0.1");
    let mut command = String::from("powerOff");
    let mut sub = String::from("keypress");
    let mut show_list_commands = false;
    let mut listen = false;

    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut port)
            .add_option(&["-p", "--port"], Store, "The port of Roku TV usually 8060");
        parser.refer(&mut target)
            .add_option(&["-t", "--target"], Store, "The IP Address of Roku TV");
        parser.refer(&mut command)
            .add_option(&["-c", "--command"], Store, "Command to send to Roku TV");
        parser.refer(&mut sub)
            .add_option(&["-s", "--subdomain"], Store, "the subdomain of the developer site");
        parser.refer(&mut show_list_commands)
            .add_option(&["-L", "--list-commands"], StoreTrue, "Lists commands for the TV");
        parser.refer(&mut listen)
            .add_option(&["-l", "--listen"], StoreTrue, "Start listening and open Wireshark");

        parser.parse_args_or_exit();
    }

    match (show_list_commands, listen) {
        (true, _) => list_commands(),
        (_, true) => wireshark_roku_capture(),
        _ => send_command(&target, &port, &sub, &command).await?,
    }

    Ok(())
}

fn banner() {
    println!("{}", r"
__________        __                   _________                __                .__   
\______   \ ____ |  | ____ __          \_   ___ \  ____   _____/  |________  ____ |  |  
 |       _//  _ \|  |/ /  |  \  ______ /    \  \/ /  _ \ /    \   __\_  __ \/  _ \|  |  
 |    |   (  <_> )    <|  |  / /_____/ \     \___(  <_> )   |  \  |  |  | \(  <_> )  |__
 |____|_  /\____/|__|_ \____/           \______  /\____/|___|  /__|  |__|   \____/|____/
        \/            \/                       \/            \/                         
                                 By: n0vA
    ".color(Colors::GreenFg));
}

async fn send_command(target: &str, port: &str, sub: &str, command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target_url = format!("http://{}:{}/{}/{}", target, port, sub,  command);
    let client = Client::new();
    let res = client.post(&target_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    println!("Response: {}", res.status());
    Ok(())
}

fn roku_request(file_path: &str) -> io::Result<()> {
    println!("{}","Creating Roku Request . . .".color(Colors::BrightYellowFg));
    if fs::metadata(file_path).is_ok() {
        println!("File exists");
    } else {

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path)?;

        writeln!(file, "M-SEARCH * HTTP/1.1")?;
        writeln!(file, "Host: 239.255.255.250:1900")?;
        writeln!(file, "Man: \"ssdp:discover\"")?;
        writeln!(file, "ST: roku:ecp\n")?;
    }

    Ok(())
}

fn ncat_request() -> io::Result<()> {
    roku_request("roku_request.txt");

    println!("{}", "Sending Ncat command for capture of Roku TV traffic...".color(Colors::BrightGreenFg));
    let nrequest = "ncat -u 239.255.255.250 1900 < roku_request.txt";
    
    let mut com = Command::new("sh")
        .arg("-c")
        .arg(nrequest)
        .spawn()
        .expect("Failed to run Ncat");
    
    Ok(())
}



fn wireshark_roku_capture() {
    let _ = ncat_request();

    println!("{}", "Opening Wireshark for capture of Roku TV traffic...".color(Colors::BrightBlueFg));

    let f = "-f";
    let filter_command = "tcp.port == 1900 || udp.port == 1900";
    
    let com = Command::new("wireshark")
        .arg(f)
        .arg(filter_command)
        .spawn()
        .expect("Failed to start Wireshark");
}

fn list_commands() {
    println!("Available commands for Roku TV:\n");
    println!("
    Keypress: \t Query:

    Home      \t chanperf
    Rev       \t r2d2-bitmaps
    Fwd       \t sgnodes
    Play      \t sgrendevous
    Select    \t sgrendezvous/track
    Left      \t sgrendezvous/untrack
    Right     \t registry/dev
    Down      \t tv-active-channel
    Up        \t tv-channel
    Back      \t media-player
    Info      \t device-info
    InstantReplay    
    Backspace
    Search
    Enter
    FindRemote
    VolumeDown
    VolumeMute
    VolumeUp
    PowerOff
    ChannelUp
    ChannelDown
    InputTuner
    InputHDMI1
    InputHDMI2
    InputHDMI3
    InputHDMI4
    InputAV1
    Lit_[Key]\n");

}