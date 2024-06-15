use std::io;

fn main() {
    println!("Temperature convertor!");

    loop {
        let mut num = String::new();
        let mut scale = String::new();
        
        println!("\nType number: ");
        io::stdin().read_line(&mut num).expect("Can`t read!");
        
        println!("\nType scale (C/F): ");
        io::stdin().read_line(&mut scale).expect("Can`t read!");

        let num: i32 = num.trim().parse().expect("Error");
        let scale = scale.trim();

        let final_result = change_scale(num, scale);

        let final_scale = match scale.trim() {
            "C" => String::from("F"),
            "F" => String::from("C"),
            _ => String::from("Error")
        };

        if final_result != 0 && final_scale.as_str() != "Error" {
            println!("\n{num}° {scale} is {final_result}° {final_scale}");
        } else {
            println!("\nError!");
            continue;
        }

        let mut rerun = String::new();

        println!("\nRepeat? (y/n): ");
        io::stdin().read_line(&mut rerun).expect("Can`t read!");

        match rerun.trim() {
            "y" => continue,
            _ => break
        }
    }
}

fn change_scale(n: i32, sys: &str) -> i32 {
    let result: i32 = match sys {
        "C" => n * 9 / 5 + 32,
        "F" => (n - 32) * 5 / 9,
        _ => 0
    };

    result
}