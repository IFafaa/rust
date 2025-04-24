#[derive(Debug)]
struct Meet {
    name: String,
    // start_time: Option<String>,
    // end_time: Option<String>,
    // day_time: String,
}

impl Meet {
    fn new(
        name: String,
        // start_time: Option<String>,
        // end_time: Option<String>,
        // day_time: String,
    ) -> Meet {
        Meet {
            name,
            // start_time,
            // end_time,
            // day_time,
        }
    }
}

#[derive(Debug)]
struct Calendar {
    year: i32,
    meets: Vec<Meet>,
}

impl Calendar {
    fn new(year: i32) -> Calendar {
        Calendar {
            meets: Vec::new(),
            year,
        }
    }

    fn add_meet(&mut self, meet: Meet) {
        self.meets.push(meet);
    }
}

pub fn little_calendar(year: i32) {
    let mut calendar = Calendar::new(year);
    println!("This is a {} calendar: {:?}", year, calendar);
    println!("Add new meets");
    loop {
        let mut meet_name = String::new();

        println!("Enter name of meet");

        std::io::stdin()
            .read_line(&mut meet_name)
            .expect("Error reading a meet name");

        if meet_name.len() > 0 {
            calendar.add_meet(Meet::new(meet_name));
        } else {
            println!("Error, try again")
        }

        println!("Do you and finish add meets?");
        println!("Enter yes/no");

        let mut finish_choose = String::new();
        std::io::stdin()
            .read_line(&mut finish_choose)
            .expect("Error reading a finish choose");

        if finish_choose.trim().eq_ignore_ascii_case("yes") {
            break;
        }
    }

    println!(
        "This is a {} calendar with yout meets: {:?}",
        year, calendar
    );
}
