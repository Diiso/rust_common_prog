//Const are immutable, you have to precise the type.
// They cannot be the result of a function
const MAX_POITNS: u32 = 100_000;


fn main() {
    // ---------VARIABLES----------------
    // let x = 5 --> Error because immutable var, cannot change its value
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);

    // This is called shadowing
    // It is different than having an mutable variable because here, after the shadowing,
    // y is still immutable
    let y = 5;
    let y = y*5;
    println!("The value of y is: {}", y);

    // VERY IMPORTANT : with shadowing, we can change the type of the variable !
    let spaces = "These are words";
    println!("For the moment, spaces' value is: {}",spaces);
    let spaces = spaces.len();
    println!("Now, spaces' value is: {}", spaces);

    //If we run this with a mutable variable, we'll get an error because you can't change the type
    // of a mutable variable
    // let mut spaces2 = "  ";
    // spaces2 = spaces2.len();


    //-------------DATA TYPES-----------------
    //Scalars and compounds

    //Scalars : integer (default i32), floating-point numbers (default f64), booleans, char
    // If you don't explicitly say the type, Rust will assign a type automatically (like Python)
    let x = 5.0; //f64
    let x: f32 = 5.0;

    //Compounds : two primitives compound types : tuples and arrays
    //       TUPLES
    let tup = (21,2.1,14);
    let tup: (i32, f64, u8) = (21,2.1,14);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    //To access values in a tuple : with a dot "."
    let tup1 = tup.0;
    let tup2 = tup.1;
    let tup3 = tup.2;

    println!("The value of x is: {}", tup1);
    println!("The value of y is: {}", tup2);
    println!("The value of z is: {}", tup3);

    //       ARRAYS
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    //This doesn't work because an array can only withstand one type :
    // let a = [1.1,2,3];
    // let first = a[0];
    // let second = a[1];
    // println!("first: {}, second: {}", first, second);


}
