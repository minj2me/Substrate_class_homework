fn main() {
    println!("green light stays {:?} seconds", TrafficLight::Green.get_time());
    println!("yellow light stays {:?} seconds", TrafficLight::Yellow.get_time());
    println!("red light stays {:?} seconds", TrafficLight::Red.get_time());

    let array: Vec<u32> = vec![1, 3, u32::MAX, 11111];
    let result = sum_up(&array);
    match result {
        Some(good_value) => {
            println!("result is ok:{:?}", good_value);
        }
        None => {
            println!("result is None");
        }
    }

    let retangle = Rectangle { length: 5.0, width: 6.5 };
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle { length: 3.0, height: 5.5 };
    print_area(&retangle);
    print_area(&circle);
    //print_area(&triangle);
}

////////////home work 1//////////////
trait MyTime {
    fn get_time(&self) -> i32;
}

#[derive(Debug)]
enum TrafficLight {
    Green,
    Yellow,
    Red,
}

impl MyTime for TrafficLight {
    fn get_time(&self) -> i32 {
        match self {
            self::TrafficLight::Green => 10,
            self::TrafficLight::Yellow => 3,
            self::TrafficLight::Red => 8,
        }
    }
}

////////////home work 2//////////////
fn sum_up(array: &Vec<u32>) -> Option<u32> {
    let mut result: Option<u32> = Some(0);
    for num in array {
        //checked_add, 加法, 溢出后返回None
        result = result.unwrap().checked_add(*num);
        if result.is_none() {
            break;
        }
    }
    result
}

////////////home work 3//////////////
trait HasArea {
    fn area(&self) -> f64;
}

//长方型
struct Rectangle {
    length: f64,
    width: f64,
}

//圆型
struct Circle {
    //S=πr²
    radius: f64,
}

//三角型
struct Triangle {
    //S=ah/2
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

//泛型 `T` 必须实现 `HasArea`
fn print_area<T: HasArea>(t: &T) {
    println!("{:?}", t.area());
}