use std::u32;

fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut car: Car;
    let mut engine = Transmission::Manual;
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0,
        car.convertible,
        car.motor,
        car.color,
        car.age.1
    );

    //for second type of car
    engine = Transmission::AutoMatic;
    car = car_factory(String::from(colors[1]), engine, true, 20);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0,
        car.convertible,
        car.motor,
        car.color,
        car.age.1
    );
    //creating the 3rd type of car
    engine = Transmission::SemiAutimatic;
    car = car_factory(String::from(colors[2]), engine, true, 10);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0,
        car.convertible,
        car.motor,
        car.color,
        car.age.1
    );

    struct Car {
        color: String,
        age: (Age, u32),
        convertible: bool,
        motor: Transmission,
    }
    #[derive(Debug)]
    #[warn(dead_code)]
    enum Transmission {
        AutoMatic,
        SemiAutimatic,
        Manual,
    }
    #[derive(Debug)]
    enum Age {
        New,
        Used,
    }
    fn car_factory(color: String, motor: Transmission, convertible: bool, miles: u32) -> Car {
        Car {
            color: color,
            convertible: convertible,
            motor: motor,
            age: car_quality(miles),
        }
    }

    fn car_quality(miles: u32) -> (Age, u32) {
        let quality = (Age::New, miles);
        return quality;
    }
}
