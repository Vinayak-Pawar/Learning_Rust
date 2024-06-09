
fn main() {
    let mut height = 190;
    height = height - 20; // Subtract 20 from height

    // Conditional expression to determine the value of `result`
    let result = if height < 180 {
        "Tall"         // No semicolon: This is an expression returning "Tall"
    } else if height > 170 {
        "Average"      // No semicolon: This is an expression returning "Average"
    } else {
        "Short"        // No semicolon: This is an expression returning "Short"
    };

    // To see the result, you can add the following line:
    println!("The result is: {}", result);
}

