// code 5.1

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        if i == 1 {dp[i] = (h[i] - h[i - 1]).abs()}
        else {
            dp[i] = min(
                dp[i - 1] + (h[i] - h[i - 1]).abs(),
                dp[i - 2] + (h[i] - h[i - 2]).abs()
            );
        }
    }

    println!("{}", dp[n - 1]);
}
