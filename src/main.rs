mod math;
use math::calculator;

fn main() {
    match calculator::calculate("2+3") {
        Ok(result) => println!("2 + 3 = {}", result),
        Err(e) => eprintln!("Error calculating expression: {}", e),
    }
}
