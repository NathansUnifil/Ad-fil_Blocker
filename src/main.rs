// #![allow(unused)]

use std::cmp::Ordering;
use std::io;
use std::ops::Add;
use std::process::Output;
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
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("{} + {} = {}", num_3, num_4, num_3 + num_4);
    println!("{} - {} = {}", num_3, num_4, num_3 - num_4);
    println!("{} * {} = {}", num_3, num_4, num_3 * num_4);
    println!("{} / {} = {}", num_3, num_4, num_3 / num_4);
    println!("{} % {} = {}", num_3, num_4, num_3 % num_4);
    num_3 += 1;
    println!("{}", num_3);

    let random_num = rand::thread_rng().gen_range(1..401);
    println!("Random = {}", random_num);

    let age = 8;
    if (age >= 1) && (age <= 18){
        println!("Important Birthday");
    } else if (age == 21) || (age == 50){
        println!("Important Birthday");
    } else if age >= 65  {
        println!("Important Birthday");
    } else {
        println!("Unimportant Birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote : {}", can_vote);

    let age2 = 64;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX =>println!("Important Birthday"),
        _ => println!("Unimportant Birthday"),
    };

    let my_age2 = 18;
    let voting_age = 18;
    match my_age2.cmp(&voting_age) {
        Ordering::Less => println!("Não pode votar"),
        Ordering::Greater => println!("Pode votar"),
        Ordering::Equal => println!("Você tem o direito de votar"),
    }

    let arr_1 = [1,2,3,4,5,6,7,8,9];
    println!("Primeiro = {}", arr_1[0]);
    println!("Segundo = {}", arr_1[1]);
    println!("Terceiro = {}", arr_1[2]);
    println!("Quarto = {}", arr_1[3]);
    println!("Tamanho = {}", arr_1.len());

    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    /*
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx +=1;
            continue;
        }
        if arr_2[loop_idx] == 9{
            break;
        }
        println!("Valores = {}", arr_2[loop_idx]);
        loop_idx +=1;
    }

     */

    /*
    while loop_idx < arr_2.len(){
        println!("Array = {}", arr_2[loop_idx]);
        loop_idx +=1;
    }

     */

    for val in arr_2.iter() {
        println!("Valor = {}", val);
    }

    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    println!("Name: {}", my_tuple.1);
    //println!("Valores: {}", my_tuple.);
    let(v1, v2, v3) = my_tuple;
    println!("Idade = {}", v1);

    //String
    //&str

    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b n k k a n p");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Rendom string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length =  {}", st6.len());
    st5.clear();
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    for char in st8.bytes(){
        println!("{}", char);
    }

    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let in3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("{}", in3_u32);

    enum Day {
        Segunda,
        Terca,
        Quarta,
        Quinta,
        Sexta,
        Sabado,
        Domingo
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Sabado | Day::Domingo => true,
                _ => false
            }
        }
    }

    let today:Day = Day::Segunda;
    match today{
        Day::Segunda => println!("Todo mundo odeia segunda-feiras"),
        Day::Terca => println!("Dia de mandioca"),
        Day::Quarta => println!("Dia de ginasio"),
        Day::Quinta => println!("Salario"),
        Day::Sexta => println!("Quase final de semana"),
        Day::Sabado => println!("Final de semana"),
        Day::Domingo => println!("Final de semana")
    }

    println!("Is today the weekend = {}", today.is_weekend());

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd = {}", second),
        None => println!("Valor não encontrado")
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vec lenght {}", vec2.len());
    println!("Pop = {:?}", vec2.pop());

    say_hello();

    get_sum(8, 9);

    println!("{}", get_sum2(7, 7));

    let (val_1, val_2) = get_2 (3);
    println!("Nums : {} {}", val_1, val_2);

    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));

    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.7 + 4.6 = {}", get_sum_gen(5.7, 4.6));
    let str3 = String::from("World");
    //let str4 = str3;
    let str4 = str3.clone();
    print_str(str3);
    let str5 = print_return_str(str4);
}

fn say_hello() {
    println!("Olá!")
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum2(x: i32, y: i32) -> i32 { // return
    x + y
}
fn get_sum3(x: i32, y: i32) -> i32 { // return
    return x + y;
}

fn get_2(x: i32) -> (i32, i32) { // return
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 { // return
   let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn print_str(x: String) {
    println!("Uma string {}", x);
}

fn print_return_str(x: String) -> String{
    println!("Uma string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message : {}", name);
}

