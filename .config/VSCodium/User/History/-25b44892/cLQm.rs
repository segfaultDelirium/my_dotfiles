use std::collections::HashMap;

use substring::Substring;
use std::fs;

fn main() {


    zadanie1();
    zadanie2();
    zadanie3();
    zadanie4();
    zadanie5();
    // matrix();
    zadanie6();
    zadanie7();
    zadanie8();
    zadanie9();
    zadanie10();
    zadanie11();
    // zadanie12();
    // zadanie13();
    // zadanie14();
    // zadanie15();
}

fn zadanie11(){
    println!("zadanie11");

    read_names_from_file();

}

fn read_names_from_file() -> String{
    let file_content = match fs::read_to_string("names.txt"){
        Ok(v) => v,
        Err(e) => {
            panic!("error reading from file {e}")
        }
    };
    println!("file content is: \n{file_content}");
    return file_content;
}


fn zadanie10(){
    println!("zadanie10");
    let digit_to_word_mapping = create_digit_to_word_mapping();

    // uncomment for user input
    // let mut user_input = String::new();
    // std::io::stdin().read_line(&mut user_input).unwrap();
    // let number_from_user = user_input.trim();

    let number_from_user = "-123456789".to_string();

    let text = number_from_user.chars()
        .map(|x| {
            if digit_to_word_mapping.contains_key(&x){
                return digit_to_word_mapping.get(&x).unwrap().to_string();
            }
            panic!("unexpected character: {x}")
        })
        .fold(String::new(), |acc, s| concat_with_space(acc, s));
    println!("your number spelled out: {text}");
}

fn concat_with_space(s1: String, s2: String) -> String{
    return concat(concat(s1, " ".to_string()), s2);
}

fn concat(s1: String, s2: String) -> String{
    let mut s = s1.clone();
    s.push_str(&s2);
    return s;
}

fn create_digit_to_word_mapping() -> HashMap<char, String>{
    let mut digit_to_word_mapping: HashMap<char, String> = HashMap::new();
    digit_to_word_mapping.insert('0', "zero".to_string());
    digit_to_word_mapping.insert('1', "jeden".to_string());
    digit_to_word_mapping.insert('2', "dwa".to_string());
    digit_to_word_mapping.insert('3', "trzy".to_string());
    digit_to_word_mapping.insert('4', "cztery".to_string());
    digit_to_word_mapping.insert('5', "pięć".to_string());
    digit_to_word_mapping.insert('6', "sześć".to_string());
    digit_to_word_mapping.insert('7', "siedem".to_string());
    digit_to_word_mapping.insert('8', "osiem".to_string());
    digit_to_word_mapping.insert('9', "dziewięć".to_string());
    digit_to_word_mapping.insert('-', "minus".to_string());
    return digit_to_word_mapping;
}

fn zadanie9(){
    println!("zadanie9");
    for i in 1..16{
        let power_value = power(i);
        println!("power of {i} = {power_value}");
    }

    let power15 = power(15);
    // power15.to_string().chars().take(3).fold(String::new(), |acc, c|  )
    println!("first 3 digits of power(15) = {}", power15.to_string().substring(0, 3));
}

fn power(n: i64) -> i64{
    return (1..(n+1)).fold(1, |acc, x| acc * x)
}

fn zadanie8(){
    println!("zadanie8");
    matrix()
}

fn zadanie7(){
    println!("zadanie7");
    let mut results_map: HashMap<i32, i32> = HashMap::new();

    let n = 0;
    let res = fibbonacci(n, &mut results_map);
    let res2 = naive_fibbonacci(n);
    println!("result of n: {n} = {res}");
    println!("result of n: {n} = {res2}");

    for i in 0..10{
        let mut results_map: HashMap<i32, i32> = HashMap::new();
        println!("");
        let res = fibbonacci(i, &mut results_map);
        let res2 = naive_fibbonacci(i);
        println!("result of n: {i} = {res}");
        println!("result of n: {i} = {res2}");
    }
}

fn fibbonacci(n: i32, results_map: &mut HashMap<i32, i32>) -> i32{
    if n < 1 { return 0 }
    if n == 1 { return 1 }
    if results_map.contains_key(&n) {
        return *results_map.get(&n).unwrap();
    }
    let res = fibbonacci(n-1, results_map) + fibbonacci(n-2, results_map);
    results_map.insert(n, res);
    return res;
}


fn naive_fibbonacci(n: i32) -> i32{
    if n < 1 {return 0};
    if n == 1 {return 1};
    
    return naive_fibbonacci(n-1) + naive_fibbonacci(n-2);
}

fn zadanie6(){
    println!("zadanie6");

    println!("initial money after {} trips {}", 2, find_initial_money(2));
    println!("initial money after {} trips {}", 3, find_initial_money(3));
    

}

fn find_initial_money(n: i32) -> f64{

    let mut initial_money = 0.;
    for _i in 0..n{
        initial_money = 0.5 * (initial_money + 12.)
    }
    return initial_money;
}

fn matrix(){
    for i in 1..11{
        for j in 1..11{
            print!("{}\t", i * j);
        }
        println!()
    }
}

fn zadanie5(){
    fn find_amount_of_matching_numbers(n: i32) -> usize{
        return (1..(n+1))
            .filter(|x| x % 7 == 0)
            .filter(|x| (2..7).all(|y| x % y == 1)).count();
    }

    let amount_of_matching_numbers = find_amount_of_matching_numbers(100);
    println!("amount of matching numbers: {amount_of_matching_numbers}");
    println!("amount of matching numbers: {}", find_amount_of_matching_numbers(2000));

}

fn zadanie4(){

    fn find_number() -> Option<i32>{
        return (1..1000)
        .filter(|x| x % 7 == 0)
        .find(|x| (2..7).all(|y| x % y == 1) )
    }

    let res;
    match find_number(){
        Some(v) => {
            res = v;
        },
        None => {
            println!("such number was not found");
            return;
        }
    };
    println!("{res}");
}


fn zadanie3(){
    println!("zadanie3");

    fn f1(n: i64) -> i64{
        let mut s: i64 = 1;
        for k in 1..n{
            s *= k;
        }
        return s;
    }

    fn f2(n: i64) -> i64{
        let mut s: i64 = 1;
        for k in 1..n{
            s *= 2 * k + 1;
        }
        return s;
    }

    fn f3(n: i64) -> f64{
        let mut s: f64 = 1.;
        for k in 1..n{
            let k_as_f = k as f64;
            s *= 1. / (k_as_f * (k_as_f + 1.));
        }
        return s;
    }

    fn f4(n: i64) -> f64{
        let mut s: f64 = 1.;
        for k in 1..n{
            let k_as_f = k as f64;
            s *= 1. / (k_as_f * k_as_f);
        }
        return s;
    }

    fn f5(n: i64) -> i64{
        let mut s: i64 = 1;
        for k in 1..n{
            s *= k.pow(3);
        }
        return s;
    }

    let n = 5;
    println!("f1({n}) = {}", f1(n));
    println!("f2({n}) = {}", f2(n));
    println!("f3({n}) = {}", f3(n));
    println!("f4({n}) = {}", f4(n));
    println!("f5({n}) = {}", f5(n));
    


}




fn zadanie2(){
    println!("zadanie2");
    
    
    fn funny_sum(n: i32) -> i32{

        let mut sum = 0;
        print!("1 + ");
        for i in 2..(n+1){
            let number_as_string = multiply_string( i.to_string(), i);
            // println!("number as string: {number_as_string}");
            print!("{number_as_string} + ");
            sum += number_as_string.parse::<i32>().unwrap();
        }

        println!("");
        return 1 + sum;
    }

    fn multiply_string(s: String, n: i32) -> String{
        
        let mut res = s.clone();
        for _i in 1..n{
            res = format!("{}{}", res, s);
        }

        return res;
    }
    
    println!("funny sum({}) = {}", 4, funny_sum(4));
    println!("funny sum({}) = {}", 6, funny_sum(6));
}


fn zadanie1(){
    fn print_decrypt_result(text: &String, shift_count: u8, shift_right: bool){
        let custom_text = if shift_right {"right"} else {"left"};
        let decrypted_text = decrypt(text, shift_count, shift_right);
        println!("decrypted with shift count: {shift_count} to the {custom_text} = {decrypted_text}")
    }
    
    fn decrypt(text: &String, shift_count: u8, shift_right: bool) -> String{
    
        let mut decrypted_test = "".to_string();
        for c in text.chars(){
            if char::is_whitespace(c){
                decrypted_test.push(c);
                continue;
            }
            let char_as_number = c as u8;
            let shifted_char = (if shift_right {char_as_number + shift_count} else {char_as_number - shift_count} )as char;
            decrypted_test.push(shifted_char );
        }
    
        return decrypted_test;
    }

    let text = "Rmgi$ he}%".to_string();

    println!("ecrypted text is: {text}");

    for i in 1..10{
        print_decrypt_result(&text, i, true);
    }

    for i in 1..10{
        print_decrypt_result(&text, i, false);
    }
}


