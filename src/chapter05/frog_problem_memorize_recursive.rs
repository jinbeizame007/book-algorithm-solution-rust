// code 5.6

use proconio::input;
use num_traits::Num;

fn chmin<T: Num + std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b {return a} else {return b};
}

fn rec(i: usize, h: &[i32], dp: &mut Vec<i32>) -> i32 {
    if dp[i] < i32::MAX {return dp[i]};

    if i == 0 {return 0};

    let mut res = i32::MAX;
    res = chmin(res, rec(i - 1, h, dp) + (h[i] - h[i - 1]).abs());

    if i > 1 {res = chmin(res, rec(i - 2, h, dp) + (h[i] - h[i - 2]).abs())};

    dp[i] = res;
    return dp[i];
}

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;

    println!("{}", rec(n - 1, &h, &mut dp));
}
