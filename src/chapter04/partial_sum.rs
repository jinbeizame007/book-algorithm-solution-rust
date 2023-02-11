// code 4.9
use::proconio::input;

fn func(i: i32, w: i32, a: &[i32]) -> bool {
    if i == 0 {
        if w == 0 {return true}
        else {return false};
    }

    if func(i - 1, w, a) {return true};
    if func(i - 1, w - a[(i - 1) as usize], a) {return true};
    return false
}

fn main() {
    input! {
        n: i32,
        w: i32,
        a: [i32; n],
    }

    if func(n, w, &a) {println!("Yes")}
    else {println!("No")};
}
