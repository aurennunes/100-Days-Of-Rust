use std::io;

fn calc_age(age: i32) -> i32 {
    let days = 365;
    let result = age * days;

    result
}

fn main() {
    println!("Digite a sua idade:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a idade!");

    let age = input.trim().parse().expect("Idade invalida!");
    let age_in_days = calc_age(age);

    println!("{} anos de idade em dias é: {}", age, age_in_days)
}
