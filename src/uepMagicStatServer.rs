#![allow(non_snake_case)]

mod functions_db_related;
mod class_Comments;
mod class_Matches;
mod class_Players;
mod class_Mythic;
mod functions_web_related;
mod traits;

use std::{env, thread};
use std::process::exit;
use actix_web::{App, HttpServer, web};

use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

use text_colorizer::{ColoredString, Colorize};

use crate::functions_db_related::establish_connection;
use crate::functions_web_related::{actions, get_index};

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

    let serveur = HttpServer::new
      (
        ||
          {
              App::new().route("/",web::get().to(get_index)).route("/post",web::post().to(actions))
          }
      );
    println!("Serving on http://192.168.0.4:4040...");
    let signals = Signals::new(&[SIGINT]);
    if signals.is_ok()
    {
        eprintln!("[{}] Signal handling on...","INFO".bright_blue());
        thread::spawn
          (  move ||
            {
                for thesignal in signals.unwrap().forever()
                {
                    println!("Received signal {:?}",thesignal);
                }
            }
          );
    }
    serveur.bind("192.168.0.4:4040").expect("error binding server to address").run().expect("error running server");
}
