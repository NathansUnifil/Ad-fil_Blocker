// #![allow(unused)]

use std::io;
use rand::Rng;

fn main() {
    /*
    println!("Qual é seu nome?");
    let mut name =  String::new();
    let saudacoes =  "Que bom te conhecer!";
    io::stdin().read_line(&mut name)
        .expect("Sem nome!");

    println!("Hello {}, {}", name.trim_end(), saudacoes);

     */
    /*
    const ONE_MIL: u32 = 1_000_000;
    const  PI: f32 = 3.14;
    let age =  "47";
    let mut age: u32 = age.trim().parse()
        .expect("Idade não definida");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

     */

    let _is_true = true; // Esse valor não vai ser usado porque tem um "_" no começo
    let _my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("{} + {} = {}", num_3, num_4, num_3 + num_4);
    println!("{} - {} = {}", num_3, num_4, num_3 - num_4);
    println!("{} * {} = {}", num_3, num_4, num_3 * num_4);
    println!("{} / {} = {}", num_3, num_4, num_3 / num_4);
    println!("{} % {} = {}", num_3, num_4, num_3 % num_4);
}
