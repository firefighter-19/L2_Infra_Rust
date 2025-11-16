#[warn(clippy::all, clippy::pedantic)]
// // struct Color(u8, u8, u8);
#[derive(Debug, Clone)] //trait to print the struct in a human-readable format
struct Car {
    brand: String,
    max_speed: u16,
    max_gas: f32,
    current_gas: f32,
    gas_consumption: f32,
    status: OrderStatus,
}

#[derive(Debug, Clone)]
enum OrderStatus {
    Pending,
    Shipped { tracking_number: String },
    Delivered,
    Cancelled(String),
}

impl OrderStatus {
    fn is_delivered(&self) -> bool {
        matches!(self, OrderStatus::Delivered)
    }
    fn is_cancelled(&self) -> bool {
        matches!(self, OrderStatus::Cancelled(_))
    }
}

impl Car {
    fn new(
        brand: &str,
        max_speed: u16,
        max_gas: f32,
        current_gas: f32,
        gas_consumption: f32,
        status: OrderStatus,
    ) -> Self {
        Self {
            brand: String::from(brand),
            max_speed,
            max_gas,
            current_gas,
            gas_consumption,
            status,
        }
    }
    fn drive(&mut self, distance: f32) {
        let gas_consumed = distance / 100.0 * self.gas_consumption;
        self.current_gas -= gas_consumed;
        println!(
            "Driving {} km, gas consumed: {} liters",
            distance, gas_consumed
        );
    }
}

fn main() {
    let mut my_car = Car::new("Toyota", 200, 50.0, 20.0, 5.0, OrderStatus::Pending);

    let my_car_2 = Car {
        gas_consumption: 3.0,
        status: OrderStatus::Cancelled(String::from("Customer cancelled the order")),
        ..my_car.clone()
    };

    let my_car_3 = Car {
        gas_consumption: 8.0,
        status: OrderStatus::Shipped {
            tracking_number: String::from("1234567890"),
        },
        ..my_car.clone()
    };

    println!("1: {my_car:#?}");

    println!("2: {my_car_2:#?}");

    println!("3: {my_car_3:#?}");

    println!("Status: {:?}", my_car_3.status);

    let value = Some(32);

    // match value { // match is a pattern matching syntax
    //     Some(value) => println!("Value: {}", value),
    //     None => println!("No value"),
    // }

    if let Some(value) = value {
        // if let is a pattern matching syntax
        println!("Value: {}", value);
    }

    println!(
        "My car gas consumption is: {} liters per 100 km",
        my_car.gas_consumption
    );

    my_car.drive(100.0);
}
