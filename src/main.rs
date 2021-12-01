mod days;

enum Day {
    All,
    Specific(i32),
}

impl<T: AsRef<str>> TryFrom<Option<T>> for Day {
    type Error = String;

    fn try_from(s: Option<T>) -> Result<Self, Self::Error> {
        match s.map(|s| s.as_ref().parse::<i32>()) {
            None => Ok(Day::All),
            Some(Ok(n)) if (1..=25).contains(&n) => Ok(Day::Specific(n)),
            Some(Ok(_)) => Err(String::from("Day must be between 1 and 25")),
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
        .get_matches();

    match Day::try_from(matches.value_of("day")) {
        Ok(Day::All) => days::run_all(),
        Ok(Day::Specific(day)) => days::run(day),
        Err(_) => eprintln!("Invalid day"),
    }
}
