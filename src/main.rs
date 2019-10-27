use std::io;

fn main() {
    let mut memo: Vec<usize> = vec![0, 1];

    loop {
        println!("Please input a number that you want to get fibonacci");
        let input = input_number();
        memo.resize(94, 0);
        println!("{}", fibo_memo(input, &mut memo));
    }
}

fn fibo_memo(n: usize, memo: &mut Vec<usize>) -> usize {
    let mut result: usize = 0;

    if n != 0 && memo[n] == 0 {
        memo.push(0);
        memo[n] = fibo_memo(n - 1, memo) + fibo_memo(n - 2, memo);
        *&mut result = memo[n];
    } else {
        *&mut result = memo[n];
    }

    result
}

fn input_number() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().parse().unwrap();

    input
}
