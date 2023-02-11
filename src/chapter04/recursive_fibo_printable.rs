fn fibo(n: i32) -> i32 {
    println!("fibo({})を呼び出しました", n);

    if n == 0 {return 0}
    else if n == 1 {return 1};

    let result = fibo(n - 1) + fibo(n - 2);
    println!("{} 項目 = {}", n, result);

    return result;
}

fn main() {
    fibo(6);
}
