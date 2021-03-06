//Const are immutable, you have to precise the type.
// They cannot be the result of a function
const MAX_POITNS: u32 = 100_000;


fn main() {
    // ---------VARIABLES----------------
    // let x = 5 --> Error because immutable var, cannot change its value
    let mut x = 5;
    //println!("The value of x is: {}",x);
    x = 6;
    //println!("The value of x is: {}",x);

    // This is called shadowing
    // It is different than having an mutable variable because here, after the shadowing,
    // y is still immutable
    let y = 5;
    let y = y*5;
    //println!("The value of y is: {}", y);

    // VERY IMPORTANT : with shadowing, we can change the type of the variable !
    let spaces = "These are words";
    //println!("For the moment, spaces' value is: {}",spaces);
    let spaces = spaces.len();
    //println!("Now, spaces' value is: {}", spaces);

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
    //println!("The value of y is: {}", y);

    //To access values in a tuple : with a dot "."
    let tup1 = tup.0;
    let tup2 = tup.1;
    let tup3 = tup.2;

    // println!("The value of x is: {}", tup1);
    // println!("The value of y is: {}", tup2);
    // println!("The value of z is: {}", tup3);

    //       ARRAYS
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    // println!("first: {}, second: {}", first, second);

    //This doesn't work because an array can only withstand one type :
    // let a = [1.1,2,3];
    // let first = a[0];
    // let second = a[1];
    // println!("first: {}, second: {}", first, second);

    //-----------FUNCTIONS--------------
    let x = 5;
    // another_function(x);

    //Statement vs expressions
    //This is a statement
    let x = 5;

    //This is also a statement but resulting from an expression
    let y = {
        let x = 3;
        //This is an expression
        x+1
    };

    // println!("The value of y is: {} and the value of x is {}", y, x);

    //Example of a function with a return value :
    let x = 5;
    let y = 10;
    let z = muliply(x,y);
    //println!("The value of {} by {} is: {}", x, y, z);
    //In the end, remember that there is a huge difference between statements and expressions

    //-----------CONTROL FLOW----------------
    //      IF STATEMENTS
    let number = 3;

    if number < 5 {
        println!("Ok");
    } else {
        println!("Not ok");
    }

    // if in a let :
    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };
    //println!("The value of number is: {}", number);

    //Both values in the if and else MUST BE THE SAME TYPE

    //      LOOPS
    let mut k = 0;
    // loop {
    //     k = k+1;
    //     println!("k = {}", k);
    //     if k == 1000 {
    //         break;
    //     }
    // }

    // while k < 1000 {
    //     k = k+1;
    //     println!("k = {}", k);
    // }

    let T = [10,11,12,13,14];

    // for number in T.iter(){
    //     println!("The current value is: {}", number);
    // }
    //
    // for number in 0..T.len() {
    //     println!("The current value of the index {} is: {}", number, T[number]);
    // }

    let n = 40;
    let fib = fibonacci(n);
    println!("The {}th value of the fibonacci sequence is: {}", n, fib);

}

//As far as I understood, you need to input the type of the function parameters
fn another_function(x: i32) {
    println!("The value of x is: {}",x);
}

//You have to specify the type of the return value
fn muliply(x: i32, y: i32) -> i32 {
    let z = x*y;
    z
}


fn fibonacci(n: i32) -> i32 {
    if n == 1 {
        1
    } else if n == 2 {
        2
    }
    else{
        fibonacci(n-1)+fibonacci(n-2)
    }
}
