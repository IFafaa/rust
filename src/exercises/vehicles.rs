struct Vehicle {
    name: String,
    speed: f32,
}
trait VehicleTrait {
    fn create(name: String, speed: f32) -> Self;
    fn acelerate(&mut self, speed: f32);
    fn brake(&mut self, speed: f32);
    fn status(&self);
}

impl VehicleTrait for Vehicle {
    fn create(name: String, speed: f32) -> Self {
        Vehicle { name, speed }
    }

    fn acelerate(&mut self, speed: f32) {
        println!("Acelerating {} to {} km/h", self.name, speed);
        self.speed += speed;
    }

    fn brake(&mut self, speed: f32) {
        println!("Braking {} to {} km/h", self.name, speed);
        if speed <= self.speed {
            self.speed -= speed;
        } else {
            println!("Speed is already at 0 km/h!");
        }
    }

    fn status(&self) {
        println!("{} is going at {} km/h", self.name, self.speed);
    }
}

pub fn vehicles() {
    let mut motorcycle = Vehicle::create(String::from("Honda CBR"), 0.0);
    let mut car = Vehicle::create(String::from("Fusca"), 0.0);
    let mut truck = Vehicle::create(String::from("Volvo"), 0.0);

    motorcycle.acelerate(100.0);
    motorcycle.status();
    motorcycle.brake(50.0);
    motorcycle.status();

    car.acelerate(80.0);
    car.status();
    car.brake(30.0);
    car.status();

    truck.acelerate(60.0);
    truck.status();
    truck.brake(20.0);
    truck.status();
}
