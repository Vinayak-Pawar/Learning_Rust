fn main() {
    let message = "Vinayak Pawar's Weight: ";
    let weight = 2000.0;

    weight = 1800.0;
    message = "vinayakp" // This would cause an error because `weight` is immutable
}
