struct MyParameterContainer {
    x: u8,
    y: u8,
}

/**
 * Adds the two numbers.
 */
fn add_bare(x: u8, y: u8) -> u8 {
    x + y
}

fn add_params(params: MyParameterContainer) -> u8 {
    params.x + params.y
}

fn main() {
    let x = 42;
    let y = 23;

    let params = MyParameterContainer { x: 42, y: 23 };

    println!("{} + {} = {}", x, y, add_bare(x, y));
    println!("{} + {} = {}", x, y, add_params(params));
}
