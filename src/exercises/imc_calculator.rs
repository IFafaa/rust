const IMC_UNDERWEIGHT: f32 = 18.5;
const IMC_NORMAL: f32 = 24.9;
const IMC_OVERWEIGHT: f32 = 29.9;
const IMC_OBESE: f32 = 40.0;

pub struct Person{
    height_in_cm: f32,
    weight_in_kg: f32,
}

pub trait ImcCalculator {
    fn create(height_in_cm: f32, weight_in_kg: f32) -> Person;
    fn imc(&self) -> f32;
}

impl ImcCalculator for Person{
    fn create(height_in_cm: f32, weight_in_kg: f32) -> Person{
        Person{
            height_in_cm,
            weight_in_kg,
        }
    }

    fn imc(&self) -> f32{
        self.weight_in_kg / ((self.height_in_cm / 100.0).powi(2))
    }
}

pub fn imc_calculator() {
    let mut height_in_cm = String::new();
    let mut weight_in_kg =  String::new();

    println!("Enter your height in centimeters: ");
    std::io::stdin()
        .read_line(&mut height_in_cm)
        .expect("Error reading your height");

    let height_in_cm = height_in_cm.trim().parse().expect("Please enter a valid number!");

    println!("Enter your weight in kg: ");
    std::io::stdin()
        .read_line(&mut weight_in_kg)
        .expect("Error reading your weight");

    let weight_in_kg = weight_in_kg.trim().parse().expect("Please enter a valid number!");

    println!("Your height is: {}", height_in_cm);
    println!("Your weight is: {}", weight_in_kg);

    let person = Person::create(height_in_cm, weight_in_kg);
    let imc = person.imc();

    println!("Your IMC is: {}", imc);

    if imc < IMC_UNDERWEIGHT {
        println!("You are underweight.");
    } else if imc < IMC_NORMAL {
        println!("You are normal weight.");
    } else if imc < IMC_OVERWEIGHT {
        println!("You are overweight.");
    } else if imc < IMC_OBESE {
        println!("You are obese.");
    } else {
        println!("You are morbidly obese.");
    }

    std::thread::sleep(std::time::Duration::from_secs(10));
}
