use std::path::Path;

mod days;
mod utils;

enum Day {
    All,
    Specific(days::Day),
}

impl<T: AsRef<str>> TryFrom<Option<T>> for Day {
    type Error = String;

    fn try_from(s: Option<T>) -> Result<Self, Self::Error> {
        match s.map(|s| s.as_ref().parse::<i32>().map(days::Day::new)) {
            None => Ok(Day::All),
            Some(Ok(Some(n))) => Ok(Day::Specific(n)),
            Some(Ok(None)) => Err(String::from("Day must be between 1 and 25")),
            _ => Err(String::from("Invalid day")),
        }
    }
}

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("NUM")
                .help("A specfic day to execute")
                .validator(|v| Day::try_from(Some(v)).map(|_| ()))
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("input")
                .short("f")
                .long("file")
                .value_name("INPUT")
                .help("File to be used as input, or\"-\" for stdin")
                .validator(|v| {
                    if v == "-" || Path::new(&v).exists() {
                        Ok(())
                    } else {
                        Err(String::from("File does not exist"))
                    }
                })
                .takes_value(true),
        )
        .get_matches();

    let input = match matches.value_of("input") {
        None => days::Input::Predefined,
        Some("-") => days::Input::Stdin,
        Some(p) => days::Input::File(p.into()),
    };

    match Day::try_from(matches.value_of("day")) {
        Ok(Day::All) => days::run_all(),
        Ok(Day::Specific(day)) => days::run(day, input),
        Err(_) => eprintln!("Invalid day"),
    }
}
