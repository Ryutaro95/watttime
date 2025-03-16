use std::io;
use watttime::calculate_heating_time;

pub fn main() {
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
    let (new_minutes, new_seconds) = calculate_heating_time(original_watt, original_time, new_watt);

    println!("適切な加熱時間: {}分{:.0}秒", new_minutes, new_seconds);
}

