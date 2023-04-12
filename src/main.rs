use std::env;
use std::process::Command;
use url::Url;

const ALLOW_LIST: [&str; 3] = ["redhat.com", "meet.google.com", "docs.google.com"];

fn pick(url: &String) -> bool {
    Url::parse(url).map_or(false, |u| {
        let domain = u.domain().unwrap_or(url.as_str());
        ALLOW_LIST.iter().any(|&s| domain.ends_with(s))
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];

    let default = Command::new("/usr/bin/firefox");

    let mut alternate = Command::new("/opt/brave.com/brave/brave-browser");
    alternate.arg("--profile-directory=Profile 1");

    if pick(url) { alternate } else { default }
        .arg(url)
        .spawn()
        .unwrap();
}
