const fn sum_of_multiples(limit: u32) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    while i < limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        i += 1;
    }
    return sum
}

const SUM: u32 = sum_of_multiples(1000);

fn main() {
    println!("{}", SUM);
}
