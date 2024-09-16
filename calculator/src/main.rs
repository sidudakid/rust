use std::io;
use colored::*;
fn main() {
    fn sum(a: u32, b: u32) -> u32{
        return a+b; 
    }
    fn diff(a: u32, b: u32)-> u32{
        return a - b;
    }   
    fn product(a: u32, b: u32) -> u32{
        return a * b;
    }
    fn div(a: u32, b: u32) -> u32{
        return a / b;
    }

    let intro = format!("1: Add.        4: product
    2: Difference   5: comming soon 
    3: Division     6: exit
");

loop{
    println!("{}","[!] Welcome to calculator [!]".blue());

    let mut readline = String::new();
    println!("Please pick one:");
    println!("{}", intro);
    io::stdin().read_line(&mut readline).expect("No input found");
    let readline: u32 = match readline.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    let mut a = String::new();
    let mut b = String::new();
    println!("Enter the first one: ");
    io::stdin().read_line(&mut a).expect("Frist number not found");
    println!("Enter the second one: ");
    io::stdin().read_line(&mut b).expect("Second number not found");
    let a: u32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    let b: u32 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    let mut pause = String::new();
    if readline == 1{
        println!("{}","Addition".green());
        println!("{}",sum(a, b));
        println!("please hit enter or any other key to continue");
        io::stdin().read_line(&mut pause).expect("Addition");
    }else if readline == 2 {
        println!("{}", "Substraction".green());
        println!("{}", diff(a,b));
        println!("please hit enter or any other key to continue");
        io::stdin().read_line(&mut pause).expect("Substraction");
    }else if readline == 3{
        println!("{}", "Division".green());
        println!("{}", div(a,b));
        println!("please hit enter or any other key to continue");
        io::stdin().read_line(&mut pause).expect("Division");
    }else if readline == 4{
        println!("{}", "Product".green());
        println!("{}",product(a,b));
        println!("please hit enter or any other key to continue");
        io::stdin().read_line(&mut pause).expect("Product");
    }else if readline == 5{
        println!("{}", "comming Soon".green());
        println!("please hit enter or any other key to continue");
        io::stdin().read_line(&mut pause).expect("Comming soon");
    }else if readline == 6{
        println!("{}", "[-] Exiting ...".red());
        break;
    }else{
        println!("{}", "[-] Error [-]".red());
        break;
    }
}

}
