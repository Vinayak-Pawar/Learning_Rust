fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

fn main() {
    let mut num = 2; // Start from 2
    let start_time = std::time::Instant::now();
    let mut prime_count = 0;

    println!("Calculating prime numbers...");

    loop {
        if is_prime(num) {
            prime_count += 1;
        }
        num += 1;

        // Check if one hour has elapsed
        let elapsed_time = start_time.elapsed();
        if elapsed_time.as_secs() >= 3600 {
            break;
        }
    }

    println!("Finished! Found {} prime numbers.", prime_count);
}
