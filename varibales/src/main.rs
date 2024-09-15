fn main() {
    let mut x = 5;
    println!("The value of xis : {}", x);
    x=6;
    println!("The value of x is {}", x);
    const NETWORTH: u32 = 100000;
    println!("networth {}", NETWORTH);
    
    let immutable = 5;
    // redecalring immutabel variable will cause error
    let string = "i am a string";

    println!("This is the string: {}", string);

    //constants are valid for entire program runs, within the scoppe they're declared in 
    //its like hardcoding the variablee
    const PI:f32 = 3.14;
    const MAX_POINTS:u32 = 100_000;

    //Shadowing
    //Shadowing means we can declare new variable with the same name as a previous variable , and
    //the new variable shadows the previous variable. for shadowingg we must use the let keyword
    fn shadow(){
        let s = 5;
        let s = s +1;
        let s = s * 2;
        println!("The value of s is: {}", s);
        println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    
    }
    shadow();

    let spaces = '    ';
    let spaces = spaces.len();

    println!("The spaces are: {}", spaces);
    // if we try to mut the spaces varible we get an error telling us we are net allowed to mutate
    // a variable types


}


