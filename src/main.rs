use std::{env, ops::Add};

fn main() {

    let args: Vec<String> = env::args().collect();

    let start_result = &args[1].to_string().parse::<u32>().unwrap();
    let start = match start_result {
        0 | 1 => 2,
        v => *v
    };

    let limit_result = &args[2].to_string().parse::<u32>().unwrap();
    let limit = match limit_result {
        check if check < &start || check < &u32::MAX => start,
        v => *v
    };

    let mut results = String::from("");

    for i in start..=limit {

        let mut is_prime = true;
        let mut first = 2;

        while first < i {
            
            if i % first == 0 {
                is_prime = false;
                break;
            }

            first += 1;
        }

        if is_prime {
            results.push_str(&first.to_string().add(", "));
        }
    }

    results.pop();
    results.pop();

    if results.is_empty() {
        println!("Kosong");
    } else {
        println!("Bilangan prima dari {}~{} adalah \n{}", start_result, limit_result, results);
    }
}