use clap::{self, App, Arg, SubCommand};
use fs_extra::dir::{self, CopyOptions};
use std::{env, fs, path::Path, process::exit};

const SATYSFI: &str = "saty";
const SATY_DIR: &str = "dir";
const TEMPLATE_DIR: &str = "TOOLBOX_TEMPLATE_DIR";

fn main() {
    let app = new_app();
    // let a = "~/.template".to_string();

    let matches = app.get_matches();

    if let Some(args) = matches.subcommand_matches(SATYSFI) {
        if let Some(target_dir_str) = args.value_of(SATY_DIR) {
            dbg!();
            let template_dir_str =
                env::var(TEMPLATE_DIR).expect(&format!("{} is not set", TEMPLATE_DIR));
            let template_dir_str = format!("{}/satysfi/", template_dir_str);
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

            println!("{:?}", target_dir);
            match fs::create_dir(target_dir) {
                Ok(_) => (),
                Err(e) => {
                    dbg!("{}", template_dir);
                    dbg!("{}", target_dir);
                    eprintln!("err {}", e);
                    exit(1);
                }
            }
            let mut options = CopyOptions::new();
            options.content_only = true;

            match dir::copy(template_dir, target_dir, &options) {
                Ok(_) => {
                    println!(
                        "successfully copied from {} to {}",
                        template_dir_str, target_dir_str
                    )
                }
                Err(e) => {
                    dbg!("{}", template_dir);
                    dbg!("{}", target_dir);
                    eprintln!("err {}", e);
                    exit(1);
                }
            }
        };
    }
}

fn new_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("toolbox")
        .version("0.1.0")
        .author("Hosshii <sao_heath6147.wistre@icloud.com>")
        .about("my toolbox")
        .subcommand(
            SubCommand::with_name(SATYSFI)
                .about("create satysfi template")
                .arg(
                    Arg::with_name(SATY_DIR)
                        .help("directory where make template")
                        .required(true),
                ),
        );
    app
}
