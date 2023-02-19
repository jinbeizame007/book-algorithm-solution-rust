// code 5.3

use proconio::input;
use num_traits::Num;

fn chmin<T: Num + std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b {return a} else {return b};
}

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = chmin(dp[i], dp[i - 1] + (h[i] - h[i - 1]).abs());
        if i > 1 {
            dp[i] = chmin(dp[i], dp[i - 2] + (h[i] - h[i - 2]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
