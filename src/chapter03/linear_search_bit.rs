use proconio::input;

fn main() {
    input! {
        n: i32,
        w: i32,
        a: [i32; n],
    }

    let mut exist = false;
    for bit in 0..(1 << n) {
        let mut sum = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += a[i as usize]
            }
        }

        if sum == w {
            exist = true;
        }
    }

    if exist {
        println!("Yes");
    } else {
        println!("No");
    }
}
