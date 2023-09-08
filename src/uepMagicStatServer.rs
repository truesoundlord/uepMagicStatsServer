#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

mod functions_db_related;
mod class_Comments;
mod class_Matches;
mod class_Players;
mod class_Mythic;
mod functions_web_related;
mod traits;
mod statics;

use std::{env, thread};
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::os::fd::AsRawFd;
use std::process::exit;
use actix_web::{App, HttpServer, web};
use console::Term;
use termios::os::linux::{ICANON, ECHO, ECHOCTL};
use termios::tcsetattr;


use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

use text_colorizer::{ColoredString, Colorize};

use crate::functions_db_related::establish_connection;
use crate::functions_web_related::{actions, get_index};
use crate::statics::LOGPATH;

fn main()
{
    let NomProgramme = env::args().next();
    println!("{} (test) 2023 RUST {}",NomProgramme.unwrap().replace("./","").as_str().bold().italic(),rustc_version::version().unwrap().to_string());

    let connection = &mut establish_connection();
    match connection
    {
        Ok(_) => println!("{}", ColoredString::from("Connected to the database...").truecolor(255, 170, 127).bold()),
        Err(oupscaca) =>
            {
                eprintln!("Erreur :{{ {}", ColoredString::from(oupscaca.to_string().as_str()).truecolor(255, 85, 0).bold());
                if oupscaca.to_string().contains("error 111")
                {
                    eprintln!("{} systemctl start mysql", "Clue:".bright_blue());
                    eprintln!("{} check /etc/my.cnf for networking !!","Clue:".bright_blue());
                    exit(-2);
                }
            },
    };

    let folder = std::fs::create_dir(LOGPATH);
    if folder.is_err()
    {
        eprintln!("[Mkdir {}] {}","ERROR".red().bold().blink(),folder.unwrap_err());
    }

    let file = OpenOptions::new().create(true).open(format!("{}{}",LOGPATH,"/server_log.log"));
    if file.is_err()
    {
        eprintln!("[Create {}] {}","ERROR".red().bold().blink(),file.as_ref().unwrap_err());
    }

    let serveur = HttpServer::new
      (
        ||
          {
              App::new().route("/",web::get().to(get_index)).route("/post",web::post().to(actions))
          }
      );
    println!("Serving on http://192.168.0.4:4040...");
    file.unwrap().write("[START]".as_bytes());

    let terminal = Term::stdout();
    terminal.hide_cursor();

    let mut configterminal = termios::Termios::from_fd(stdout().as_raw_fd());
    if configterminal.is_ok()
    {
        let configset = configterminal.as_ref().unwrap().c_lflag;
        configterminal.as_mut().unwrap().c_lflag &= !(ICANON|ECHO|ECHOCTL);

        let result = tcsetattr(stdout().as_raw_fd(), termios::TCSANOW,configterminal.as_ref().unwrap()).is_ok();
        if !result
        {
            eprintln!("[tcsetattr {}] failure !!","ERROR".red().bold().blink());
        }
    }

    let signals = Signals::new(&[SIGINT]);
    if signals.is_ok()
    {
        println!("[{}] Signal handling on...","INFO".bright_blue());
        thread::spawn
          (  move ||
            {
                for thesignal in signals.unwrap().forever()
                {
                    eprintln!("Received signal {:?}",thesignal);
                    let configset = configterminal.as_ref().unwrap().c_lflag;
                    configterminal.as_mut().unwrap().c_lflag |= ICANON|ECHO|ECHOCTL;

                    let result = tcsetattr(stdout().as_raw_fd(), termios::TCSANOW,configterminal.as_ref().unwrap()).is_ok();
                    if !result
                    {
                        eprintln!("[tcsetattr {} signal handle] failure !!","ERROR".red().bold().blink());
                    }
                    terminal.show_cursor();
                }
            }
          );
    }
    serveur.bind("192.168.0.4:4040").expect("error binding server to address").run().expect("error running server");
}
