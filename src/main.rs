mod r1_100;

fn main() {
    let problem = 2;
    match problem {
        1 => r1_100::p1::main(),
        2 => r1_100::p2::main(),
        _ => (),
    }
}
