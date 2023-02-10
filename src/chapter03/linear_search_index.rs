use proconio::input;

fn main() {
    input! {
        n: i32,
        v: i32,
        a: [i32; n],
    }

    let mut found_id = -1;
    for i in 0..n {
        if a[i as usize] == v {
            found_id = i;
            break;
        }
    }

    println!("{}", found_id);
}
