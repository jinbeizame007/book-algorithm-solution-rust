fn func(n: i32) -> i32 {
    println!("func({})を呼び出しました", n);

    if n == 0 {return 0};

    let result = n + func(n - 1);
    println!("{}までの和 = {}", n, result);

    return result;
}

fn main() {
    func(5);
}
