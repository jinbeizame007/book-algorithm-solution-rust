use proconio::input;

fn main() {
    input! {
        n: i32,
        v: i32,
        a: [i32; n],
    }

    let mut exist = false;
    for i in 0..n {
        if a[i as usize] == v {
            exist = true;
        }
    }

    if exist {
        println!("Yes");
    } else {
        println!("No");
    }
}
