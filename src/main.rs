fn main() {
    let car_1 = car_factory(String::from("Red"), Transmission::Manual, true, 1);

    println!(
        "The type of the car is {:?} and the color is {} and the convertible is {} and the mileage is: {}",
        car_1.transmission,
        car_1.color,
        car_1.convertible,
        car_1.mileage
    );

    //for second type of car
    let car_2 = car_factory(String::from("Yellow"), Transmission::AutoMatic, false, 0);
    println!(
        "The type of the car is {:?} and the color is {} and the convertible is {} and the mileage is: {}",
        car_2.transmission,
        car_2.color,
        car_2.convertible,
        car_2.mileage
    );
    //creating the 3rd type of car
    let car_3 = car_factory(String::from("Green"), Transmission::SemiAutimatic, false, 12);
    println!(
        "The type of the car is {:?} and the color is {} and the convertible is {} and the mileage is: {}",
        car_3.transmission,
        car_3.color,
        car_3.convertible,
        car_3.mileage
    )
}

struct Car {
    color: String,
    mileage: u32,
    convertible: bool,
    transmission: Transmission,
}
#[derive(Debug)]
#[warn(dead_code)]
enum Transmission {
    AutoMatic,
    SemiAutimatic,
    Manual,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool, mileage: u32) -> Car {
    Car {
        color: color,
        convertible: convertible,
        transmission: transmission,
        mileage: mileage,
    }
}
