mod day01;

const DAYS: [Day; 1] = [Day(1)];

pub struct Day(i32);
impl Day {
    pub fn new(n: i32) -> Option<Self> {
        return if (1..=25).contains(&n) {
            Some(Self(n))
        } else {
            None
        };
    }
}

fn get_day(day: Day) -> fn() {
    match day {
        Day(1) => day01::run,
        _ => unimplemented!(),
    }
}

pub fn run(day: Day) {
    get_day(day)();
}

pub fn run_all() {
    for day in DAYS {
        run(day)
    }
}
