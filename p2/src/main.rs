fn get_fibonaci_below_limit(limit: u32) -> Vec<u32> {
    let mut sequence: Vec<u32> = vec![1];
    let mut a: u32 = 1;
    let mut b: u32 = 2;
    while b < limit {
        sequence.push(b);
        (a, b) = (b, a + b);
    }
    sequence
}

fn sum_vec(list: Vec<u32>) -> u32 {
    list.iter()
        .filter(|&x| x % 2 == 0)
        .sum()
}

fn main() {
    let list = get_fibonaci_below_limit(4_000_000);
    let sum = sum_vec(list);
    println!("{}", sum);
}