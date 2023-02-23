// code 5.7

use proconio::input;
use num_traits::Num;

fn chmax<T: Num + std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {return a} else {return b};
}

fn main() {
    input! {
        n: usize,
        w: usize,
        weight: [usize; n],
        value: [usize; n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        for _w in 0..=w {
            if (_w as i32) - (weight[i] as i32) >= 0 {
                dp[i + 1][_w] = chmax(dp[i + 1][_w], dp[i][_w - weight[i]] + value[i]);
            }

            dp[i + 1][_w] = chmax(dp[i + 1][_w], dp[i][_w]);
        }
    }

    println!("{}", dp[n][w]);
}
