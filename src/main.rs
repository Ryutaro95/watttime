use std::io;

fn main() {
    let mut input = String::new();

    println!("元のワット数(w)を入力してください: ");
    io::stdin().read_line(&mut input).unwrap();
    let original_watt: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("元の加熱時間(分)を入力してください");
    io::stdin().read_line(&mut input).unwrap();
    let minutes: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("元の加熱時間(秒)を入力してください");
    io::stdin().read_line(&mut input).unwrap();
    let seconds: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("変更後のワット数(w)を入力してください");
    io::stdin().read_line(&mut input).unwrap();
    let new_watt: f64 = input.trim().parse().unwrap();

    let original_time = (minutes * 60.0) + seconds;
    let new_time = original_time * (original_watt / new_watt);

    let new_minutes = (new_time / 60.0).floor();
    let new_seconds = new_time % 60.0;

    println!("適切な加熱時間: {}分{:.0}秒", new_minutes, new_seconds);
}

