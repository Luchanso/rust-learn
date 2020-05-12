use std::io;

fn main() {
    let prime_number = calc_prime(get_user_input());

    println!("is prime: {:?}", prime_number);
}


fn get_user_input() -> u32 {
    let mut user_input = String::new();

    println!("Enter number:");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    return user_input.trim().parse().expect("Please type number!");
}

// TODO: remove usize everywhere
// TODO: write benchmarks
fn calc_prime(n: u32) -> Vec<u32> {
    let mut init_numbers: Vec<u32> = (0..n + 1).collect();
    init_numbers[1] = 0;
    let mut result: Vec<u32> = Vec::new();

    let mut i = 2;
    while i <= n {
        if init_numbers[i as usize] != 0 {
            result.push(init_numbers[i as usize]);

            for j in (i..n + 1).step_by(i as usize) {
                init_numbers[j as usize] = 0;
            }
        }
        i += 1;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::calc_prime;

    #[test]
    fn should_return_prime_number_before_ten() {
        assert_eq!(calc_prime(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn should_return_25_prime_numbers() {
        assert_eq!(calc_prime(100).len(), 25);
    }

    #[test]
    fn should_correct_works_on_1000_primes_numbers() {
        calc_prime(1000).len();
    }
}