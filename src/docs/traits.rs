fn traits() {
    // TRAITS:
    /*
    Traits are a way to define shared behavior in Rust.
    They allow you to define methods that can be implemented by different types.
    Traits are similar to interfaces in other languages.
    */
    // Example of a trait:
    trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    // Implementing the trait for a struct:
    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
        fn perimeter(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }
}