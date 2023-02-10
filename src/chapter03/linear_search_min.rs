use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }

    let mut min_value = std::i32::MAX;
    for i in 0..n {
        if a[i as usize] < min_value {
            min_value = a[i as usize];
        }
    }

    println!("{}", min_value);
}
