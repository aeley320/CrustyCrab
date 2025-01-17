// Ignore Warnings
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(dead_code)]

use std::{fs, process};
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket, TcpListener, TcpStream, Shutdown};
use std::process::{Command, Output};
use rand::Rng;
use regex::Regex;
use log::{info, warn, error, debug};
use crabby_patty_formula::*;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::mem::drop;
use std::env::{self, current_dir};
use std::path::Path;

extern crate directories;
use directories::UserDirs;

extern crate rustyline;
use rustyline::{history, hint, completion, highlight, line_buffer};


fn main() {
    // clear console first
    Command::new("clear").status().unwrap();
    // print the super cool banner
    banner();

    //Vec<(ID, PORT, PROTOCOL, )>
    let mut listen_tracker: Vec<(u64, u16, String/* , Arc<Mutex<SharedBuffer>>, u16*/)> = Vec::new();
    let mut relay_port = 2000;
    let mut sb_arc = Arc::new(Mutex::new(SharedBuffer {
        cc: 0,
        buff: [0; 2048].to_vec(),
    }));
    //Defaults
    let mut protocol: u16 = 1;
    let mut listen_port: u16 = 2120;
    let mut local_address = SocketAddr::from(([127, 0, 0, 1], listen_port));
    // main program loop
    loop {
        // print the prompt and read in a command
        // !TODO Make main shell more posix compliant

        io::stdout().flush().unwrap();
        let mut rust_inline = rustyline::DefaultEditor::new();
        let mut usr_cmd = String::new();
        let mut usr_cmd = rust_inline.expect("failed to execute process").readline("CrustyCrab $ ").unwrap();

        // regex that removes multiple spaces
        let re = Regex::new(r"\s+").unwrap();
        usr_cmd = re.replace_all(&usr_cmd, " ").to_string();
        //let re = Regex::new(r"");


        // allows user to execute multiple commands in a single line seperated by ;
        let mut cmds = usr_cmd.split(';');

        // loop through each command given
        let mut head = cmds.next();
        while head != None {
            let current_cmd = head.unwrap().trim();
            let mut output: Output = Command::new("printf").arg("").output().expect("");

            if current_cmd.eq("exit") || current_cmd.eq("quit") || current_cmd.eq("q"){ 
                // quit the program
                process::exit(0);
            } 
            else if current_cmd.starts_with("help") { 
                // print the help menu
                help();
            }
            else if current_cmd.eq("banner") { // print the banner
                banner();
            }
            else if current_cmd.eq("listen") {
                println!("[+] Opening Crusty Crab");
                info!("[+] Opening Crusty Crab");

                //Passes vector of listeners and current port
                sb_arc = Arc::clone(&open_crusty_crab(&mut listen_tracker, listen_port, local_address, protocol));
            }
            else if current_cmd.starts_with("cd") {
                if current_cmd.len() > 3{
                    let mut split_cmd = current_cmd.split(" ");
                    split_cmd.next();
                    let dir = split_cmd.next().unwrap();
                    if dir.eq("~") {
                        let user = UserDirs::new().unwrap();
                        if env::set_current_dir(user.home_dir()).is_err() {
                            // Will set the directory to home if no errors are envoked.
                            print!("cd: permission denied: {}\n", user.home_dir().to_str().unwrap())
                        } 
                    }
                    else if Path::new(&dir).exists() {
                        if env::set_current_dir(&dir).is_err() {
                            // Will set the directory if no errors are envoked.
                            print!("cd: permission denied: {dir}\n")
                        }  
                    }
                    else {
                        print!("cd: no such file or directory: {dir}\n");
                    }
                }
                else {
                    let user = UserDirs::new().unwrap();
                    if env::set_current_dir(user.home_dir()).is_err() {
                        // Will set the directory to home if no errors are envoked.
                        print!("cd: permission denied: {}\n", user.home_dir().to_str().unwrap())
                    }
                }
            }
            else if current_cmd.starts_with("rmdir") {
                if current_cmd.len() > 5 {
                  let mut split_cmd = current_cmd.split(" ");
                  split_cmd.next();
                  let object = split_cmd.next().unwrap();
                  if fs::remove_dir_all(object).is_err() {
                    // Does not remove symlinks.
                    print!("rmdir: no such directory: {} does not exist\n", object);
                  }
                }
                else {
                  print!("rmdir: missing operand\n")
                }
                //TODO
            }
            else if current_cmd.starts_with("rm"){
                if current_cmd.len() > 2 {
                    let mut split_cmd = current_cmd.split(" ");
                    split_cmd.next();
                    let object = split_cmd.next().unwrap();
                    if object.starts_with("-") && object.contains("rf") {
                        if current_cmd.len() > 6 {
                            let dir = split_cmd.next().unwrap();
                            if fs::remove_dir_all(dir).is_err() {
                                // Does not remove symlinks.
                                print!("rm: No such directory: {} does not exist\n", object);
                            }
                        }
                        else {
                            print!("{}: missing operand\n", current_cmd)
                        }
                    }
                    else if fs::remove_file(object).is_err() {
                        print!("rm: {}: No such file or directory\n", object);
                    }
                    //TODO
                }
                else {
                    print!("{} missing operand\n", current_cmd)
                }
            }
            else if current_cmd.starts_with("mv") {
                let mut split_cmd = current_cmd.split(" ");
                split_cmd.next();
                let name1 = split_cmd.next().unwrap();
                let name2 = split_cmd.next().unwrap();
                if fs::rename(name1, name2).is_err() {
                    print!("\"{}\" does not exist\n", name1);
                }
            }
            else if current_cmd.starts_with("mkdir") {
                let mut split_cmd = current_cmd.split(" ");
                split_cmd.next();
                let dir = split_cmd.next().unwrap();
                if fs::create_dir_all(dir).is_err() {
                    print!("could not create {}\n", dir);
                }
            }
            else if current_cmd.starts_with("cp") {
                let mut split_cmd = current_cmd.split(" ");
                split_cmd.next();
                let file = split_cmd.next().unwrap();
                let copy = split_cmd.next().unwrap();
                if fs::copy(file, copy).is_err() {
                    print!("Cannot copy {}", file);
                }
            }
            else if current_cmd.starts_with("cat") {
                let mut split_cmd = current_cmd.split(" ");
                split_cmd.next();
                let file = split_cmd.next();
                if file != None {
                    output = Command::new("cat").arg(file.unwrap()).output().expect("failed to execute process");
                }
            }
            else if current_cmd.starts_with("ls") {
                if current_cmd.len() > 2 {
                    let mut split_cmd = current_cmd.split_whitespace();
                    split_cmd.next();
                    let last_args: Vec<&str> = split_cmd.collect();
                    output = Command::new("ls").args(last_args).output().expect("ls command failed to start");
                }
                else {
                    output = Command::new("ls").arg(current_dir().unwrap()).output().expect("ls command failed to start");
                }
            }
            else if current_cmd.eq("pwd") {
                print!("{}\n", current_dir().unwrap().to_str().unwrap())
            }
            else if current_cmd.starts_with("whoami") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("whoami").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.eq("clear") {
                Command::new("clear").status().unwrap();
            }
            else if current_cmd.eq("top") {
                Command::new("top").status().unwrap();
            }
            else if current_cmd.starts_with("which") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("which").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.starts_with("whereis") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("whereis").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.starts_with("w") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("w").args(last_args).output().expect("failed to execute process");
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
            else if current_cmd.contains("awk") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("awk").args(last_args).output().expect("failed to execute process");
                // Don't know if this actually works. // TODO
            }
            else if current_cmd.starts_with("grep") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("grep").args(last_args).output().expect("failed to execute process");
                // Currently does not work. // TODO
            }
            else if current_cmd.starts_with("sed") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("sed").args(last_args).output().expect("failed to execute process");
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
                // Don't know if this actually works. // TODO
            }
            else if current_cmd.starts_with("dig") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("dig").args(last_args).output().expect("failed to execute process");
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
            else if current_cmd.starts_with("nslookup") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("nslookup").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.starts_with("ps") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("ps").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.starts_with("uname") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("uname").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.contains("man") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("man").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.contains("ifconfig") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("ifconfig").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.starts_with("touch") {
                let mut split_cmd = current_cmd.split_whitespace();
                split_cmd.next();
                let last_args: Vec<&str> = split_cmd.collect();
                output = Command::new("touch").args(last_args).output().expect("failed to execute process");
            }
            else if current_cmd.contains("exec") {
                println!("[+] Executing command");
                info!("[+] Executing command");
            }
            else if current_cmd.contains("set") {
                // look for all commands that contain set
                let mut command = current_cmd.split(' ');

                command.next();
                let curr = command.next().unwrap().trim();
                if curr.eq("listen") {
                    // list all anchovies and get all info

                    let option = command.next().unwrap().trim();
                    let value = command.next().unwrap().trim();

                    if option.eq("port") {
                        
                        //Error check on setting port 
                        let result: Result<u16, _> = value.parse();
                        match result{
                            Ok(result) => {
                            listen_port = value.parse().unwrap();
                            local_address = SocketAddr::from(([127, 0, 0, 1], listen_port));
                            println!("[+] Setting default listener port to {}", listen_port);},
                            Err(e) => println!("Those are the wrong ingredients!"),
                        }
                    }
                    else if option.eq("protocol"){
                        match value{
                            "udp" => {protocol = 1;  
                                println!("[+] Setting default listener protocol to {}", "udp");},
                            "tcp" => {protocol = 2;
                                println!("[+] Setting default listener protocol to {}", "tcp");},
                            "http" => println!("We didn't finish making your crabby patty yet!"),
                            "dns" => println!("We didn't finish making your crabby patty yet!"),
                            &_ => println!("Those are the wrong ingredients!"),
                        
                        }
                    }
                }
                else if curr.eq("payload"){
                    println!("Sending out patty")
                }
                else if curr.eq("anchovy"){
                    // kill anchovy based on its number

                    let option = command.next().unwrap().trim();
                    let value = command.next().unwrap().trim();

                    if option.eq("ip"){
                        println!("[+] Setting anchovy server ip to {}", value);
                    }
                    else if option.eq("os"){
                        println!("[+] Setting default anchovy os to {}", value);
                    }
                    println!("sPongBOB what are you doin to me customers");
                }
            }
            else if current_cmd.contains("anchovy") {
                let mut command = current_cmd.split(' ');

                command.next();
                let option = command.next().unwrap().trim();
                let value = command.next().unwrap().trim();
                if option.eq("list") {
                    // list all anchovies and get all info
                    println!("[+] Listing all anchovies");
                    println!("Spongebob look at all the customers me boi ");
                }
                else if option.contains("select"){
                    println!("[+] Selected anchovy {}", value);
                    println!("One krabby patty coming up (anchovy select)");
                }
                else if option.eq("spawn"){
                    create_anchovy();
                }
                else if option.contains("kill"){
                    // kill anchovy based on its number
                    println!("[-] Killing anchovy");
                    println!("sPongBOB what are you doin to me customers (anchovy kill)");
                }
            }
            else if current_cmd.contains("listen") {
                let mut command = current_cmd.split(' ');

                command.next();

                let mut curr_head = command.next();
                let mut curr = curr_head.unwrap().trim();
                if curr.eq("exit") {
                    // list all anchovies and get all info
                    println!("Squidward take the trash out its time to close");
                }
                else if curr.contains("kill"){
                    let value = command.next().unwrap().trim();
                    
                    println!("Closing the register");
                }
                else if curr.eq("list"){
                    println!("\nID\tPORT\tPROTOCOL");
                    println!("------------------------------------------");
                    for listener in &listen_tracker{
                        println!("{:?}\t{:?}\t{}", listener.0, listener.1, listener.2.to_string().to_uppercase());
                    }
                    println!("------------------------------------------");
                    println!("Spongebob look at all me customers!\n");
                }
            }
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            head = cmds.next();
        }
    }
}

// prints random ascii art
fn banner(){
    let mut rng = rand::thread_rng();

    let banner = format!("static/art/banner{}.txt",rng.gen_range(0..13));
    let contents = fs::read_to_string(&banner);

    println!("{c}\n", c=contents.unwrap());
}


// prints help optional second argument for more specific details
fn help(){
    let contents = fs::read_to_string("static/help.txt");
    println!("{c}\n", c=contents.unwrap());
}


// creates implant for server ip
fn create_anchovy() {
    println!("Spongebob there's another anchovy");

    let mut binding = Command::new("sh");
    let mut result  = binding.arg("-c").arg("cargo build -q --bin implant");
}


// open listener
fn open_crusty_crab(tracker: &mut Vec<(u64, u16, String)>, relay_port: u16, address: SocketAddr, prot_type: u16) -> Arc<Mutex<SharedBuffer>>{
    // let mut local: String = "127.0.0.1:".to_owned();
    // local.push_str(&relay_port.to_string()[..]);
    // let relay = UdpSocket::bind(local).unwrap();
    // tracker.push(((tracker.len() as u64) + 1, relay_port, relay));
    
    
    //Create a new listener
    let mut new_listen = new_lsn(tracker.len() as u64);
    let mut protocol = "udp";
    if prot_type == 1{
        protocol = "udp";
        
    }
    else if prot_type == 2{
        protocol = "tcp";
    }
    tracker.push(((tracker.len() as u64) + 1, relay_port, protocol.to_string()));
    
    //Create the shared buff and clone 
    let mut sb: Arc<Mutex<SharedBuffer>> = Arc::new(Mutex::new(SharedBuffer {
        cc: 0,
        buff: [0; 2048].to_vec(),
    }));

    let mut sb_arc = Arc::clone(&sb);
    
    let thr = thread::spawn(move ||
        {
            crabby_patty_formula::lsn_run(&mut new_listen, protocol, address, &mut sb);
        }
    );
    thread::sleep(time::Duration::from_millis(10));
    return sb_arc;

}
