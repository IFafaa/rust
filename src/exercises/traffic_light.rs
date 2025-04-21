use colored::*;

enum TrafficLightColors {
    Red = 1,
    Yellow = 2,
    Green = 3,
}

struct TrafficLight {
    pub color: TrafficLightColors,
    green_time: u32,
    yellow_time: u32,
    red_time: u32,
}

impl TrafficLight {
    fn new(green_time: u32, yellow_time: u32, red_time: u32) -> Self {
        let color = TrafficLightColors::Red;
        TrafficLight {
            color,
            green_time,
            yellow_time,
            red_time,
        }
    }

    fn change_color(&mut self) {
        match self.color {
            TrafficLightColors::Red => self.color = TrafficLightColors::Green,
            TrafficLightColors::Yellow => self.color = TrafficLightColors::Red,
            TrafficLightColors::Green => self.color = TrafficLightColors::Yellow,
        }
    }
}

pub fn traffic_light() {
    let mut traffic_light = TrafficLight::new(5, 2, 5);

    fn display_message(color: &TrafficLightColors, duration: u32) {
        match color {
            TrafficLightColors::Red => println!("{}", "Stop!".red()),
            TrafficLightColors::Yellow => println!("{}", "Caution!".yellow()),
            TrafficLightColors::Green => println!("{}", "Go!".green()),
        }
        std::thread::sleep(std::time::Duration::from_secs(duration.into()));
    }

    loop {
        match traffic_light.color {
            TrafficLightColors::Red => {
                display_message(&traffic_light.color, traffic_light.red_time);
                traffic_light.change_color();
            }
            TrafficLightColors::Yellow => {
                display_message(&traffic_light.color, traffic_light.yellow_time);
                traffic_light.change_color();
            }
            TrafficLightColors::Green => {
                display_message(&traffic_light.color, traffic_light.green_time);
                traffic_light.change_color();
            }
        }
    }
}
