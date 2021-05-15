use clap::{self, App, Arg, SubCommand};
use std::{env, fs, path::Path, process::exit};

// struct MyApp {
//     cmd: String,
//     _sub_cmd: Vec<MyApp>,
//     _flag: Vec<String>,
//     _options: Vec<String>,
// }

// impl MyApp {
//     fn new(cmd_name: impl Into<String>) -> Self {
//         Self {
//             cmd: cmd_name.into(),
//             _sub_cmd: Vec::new(),
//             _flag: Vec::new(),
//             _options: Vec::new(),
//         }
//     }
//     fn sub_cmd(mut self, a: MyApp) -> Self {
//         self._sub_cmd.push(a);
//         self
//     }
//     fn option(mut self, a: impl Into<String>) -> Self {
//         self._options.push(a.into());
//         self
//     }
// }

const SATYSFI: &str = "saty";
const TEMPLATE_DIR: &str = "TOOLBOX_TEMPLATE_DIR";

fn main() {
    let app = new_app();
    // let a = "~/.template".to_string();

    let matches = app.get_matches();
    if let Some(target_dir_str) = matches.value_of(SATYSFI) {
        let template_dir_str =
            env::var(TEMPLATE_DIR).expect(&format!("{} is not set", TEMPLATE_DIR));
        let template_dir_str = format!("{}/satysfi", template_dir_str);
        let template_dir = Path::new(&template_dir_str);
        if !template_dir.exists() {
            eprintln!("{} does not exists", template_dir_str);
            exit(1);
        }

        let target_dir = Path::new(target_dir_str);
        if target_dir.exists() {
            eprintln!("{} already exist", target_dir_str);
            exit(1);
        }
        match fs::create_dir(target_dir) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("err {}", e);
                exit(1);
            }
        }

        match fs::copy(template_dir, target_dir) {
            Ok(_) => {
                println!(
                    "successfully copied from {} to {}",
                    template_dir_str, target_dir_str
                )
            }
            Err(e) => {
                eprintln!("err {}", e);
                exit(1);
            }
        }
    };
}

// fn new_my_app() -> MyApp {
//     // let satysfi = MyApp {
//     //     cmd: "satysfi".to_owned(),
//     //     sub_cmd: Vec::new(),
//     //     flag: Vec::new(),
//     //     options: vec!["dir".to_owned()],
//     // };
//     let myapp = MyApp::new("toolbox");
//     myapp
// }

fn new_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("toolbox")
        .version("0.1.0")
        .author("Hosshii <sao_heath6147.wistre@icloud.com>")
        .about("my toolbox")
        .subcommand(
            SubCommand::with_name(SATYSFI)
                .about("create satysfi template")
                .arg(
                    Arg::with_name("dir")
                        .help("directory where make template")
                        .required(true),
                ),
        );
    app
}
