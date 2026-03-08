use clap::{Arg, ArgAction, Command};

pub struct CliArgs {
    pub pid: u32,
    pub tid: u32,
    pub hide_banner: bool,
}

const BANNER: &str = r#"  _____ _             _ _______
 / ____| |           | |__   __|
| (___ | |_ __ _  ___| | _| |_ __ __ _  ___ ___ _ __
 \___ \| __/ _` |/ __| |/ / | '__/ _` |/ __/ _ \ '__|
 ____) | || (_| | (__|   <| | | | (_| | (_|  __/ |
|_____/ \__\__,_|\___|_|\_\_|_|  \__,_|\___\___|_|
                                      DB @whokilleddb"#;

impl CliArgs {
    pub fn banner() {
        // Load the standard FIGlet font
        println!("{}", BANNER);
    }

    pub fn parse() -> Self {
        let matches = Command::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .help_template(
                "{before-help}\
                {about}\n\n\
                {usage-heading} {usage}\n\n\
                {all-args}",
            )
            .before_help(BANNER)
            .arg(
                Arg::new("pid")
                    .long("pid")
                    .help("Process ID")
                    .required(true)
                    .value_parser(clap::value_parser!(u32)),
            )
            .arg(
                Arg::new("tid")
                    .long("tid")
                    .help("Thread ID (defaults to 0)")
                    .default_value("0")
                    .value_parser(clap::value_parser!(u32)),
            )
            .arg(
                Arg::new("hide-banner")
                    .long("hide-banner")
                    .help("Hide the banner")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        CliArgs {
            pid: *matches.get_one::<u32>("pid").unwrap(),
            tid: *matches.get_one::<u32>("tid").unwrap(),
            hide_banner: matches.get_flag("hide-banner"),
        }
    }
}
