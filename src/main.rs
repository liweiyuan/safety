mod math;
use math::calculator;

fn main() {
    let result = calculator::calculate("2+3");
    println!("2 + 3 = {:?}", result);
}
