fn main() {
    let mut chain_length: u32 = 0;
    let mut chain_number: u32 = 0;

    for number in 1..=1000 {
        let count = count_steps(number);
        if count > chain_length {
            chain_length = count;
            chain_number = number;
        }
    }
    println!(
        "The number {} has the longest chain: {} steps",
        chain_number, chain_length
    );
}

fn count_steps(number: u32) -> u32 {
    let mut count = 0;
    let mut number = number;
    while number != 1 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            number = (number * 3) + 1;
        }
        count += 1;
    }
    count
}
