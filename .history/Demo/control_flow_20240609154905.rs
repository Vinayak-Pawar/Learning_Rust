fn main() {
    let proceed = true;
    if proceed{
        println!("Proceeding...");
    } else {
        println!("Not Proceeding...");
    }

    let height = 190;
    if height > 180 {
        println!("You are tall");
    } else if height > 170 {
        println!("You are short");
    } else {
        println!("You are very short");
    }
}