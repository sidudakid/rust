fn main() {
    //every value in rust is of certain datatype so it knows what kind of data is being specified
    //so it knows how to work with that data. lets look two data  type subsets:
    //scalar and compoound
    //
    //Rust is statically typed language -> means it must know thee thypes of all variable at
    //compile time.
/*
    we also need to provide type annotation 
    such as:
    let guess: u32 = "42".parse().expect("Not a number.");

    here, u32 means the unsigned 32 bit integer  if we dont add this rust will throw an error.



*/

//scalar type

//A scalar type represent a single value: Rust has four: integers, floating-point numbers ,
//Boolean and characters

/*
 Interger types in rust
        8 bit = 18 -> signed = u8 -> unsigned
        16
        32 
        64
        128
        arch = isize -> signed = usize -> unsigned

 */

//numerric operation

fn calc(){
    let sum = 2 + 2;
    let sub = 3-2;
    let product = 2*2;
    let quotient = 45.3 / 22.1;
    let remainder = 23%2;

}



//Boolean type
fn Boolean_vals(){
    let t = true;
    let f: bool = false; //wiith explicit type annotation. boolean are only one byte in  size
                         
}


//The character type
let c = 'Z';
let plus = '+';
let emoji ="";

//char type is four bytes in size and represents a unicode scalar value, which means it can
//represent more than a lit more than jsust ascii . Accented letters: chinese, japanese, and korean
//characters ; emoji ; zero width spaces  are all valid char values in rust


//COMPOUND types

// -> tuple types:
// THEY HAVE FIXED LENGTH: ONCE DECLARED, THEY CANNOT GROW OR SHRINK IN SIZE

fn lrn_tuple(){
    let tup: (i32, f64, u8) = (500, 6.1, 1);
    let (x,y,z) = tup;
    println!("The value of y is: {}", y);
    println!("The first value of tuple is: {}", tup.0);
}
lrn_tuple();
// converting values of tuple to into there seperate variables is called destructuring;



// -> Array types
// Array is the collection of multiple values. unlike a tuple, every elemenet of an array must have
// the same type. In rust, arrays have a fixed length, like tuples


let arr = [1,2,3,4,5,6,7,8,9];
let months = ["Jan", "feb", "mar", "apr", "may", "jun", "jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
//now explicit annotation type:
let annona: [i32; 5] = [1,2,3,4,5];
println!("Array element firrst : {}", annona[0]);


//array allocate data in stack rather than heap
//
//note: An array isn’t as fexible as the vector type,
//though. A vector is a similar collection type provided by the standard library that is
//allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, you
//should probably use a vector.


let three_five = [3; 5];
let three_fives = [3,3,3,3,3]




}


