fn main() {
    /* Variables - immuatable by default. Mut can be used to change it*/
    let mut x = 5; 
    println!("The value of x is: {}", x);

    let x = 6;
    println!("The value of x is: {}", x);

    const _COUNT: u32 = 10;

    /*Data Types */

    let _a: i32 = 98_222; /*Decimal */
    let _b: i32 = 0xff;   /*Hex */
    let _c: i32 = 0o77;   /*Octal */
    let _d: i32 = 0b1111_0000; /*Binary */
    let _e: u8 = b'A'; /* Byte (u8 only) */

    /* Booleans */ 
    let _t: bool = true;

    let _f: bool = false;

    /* Charcter */

    let _c: char = 'z';
    let _z: char = '=';


    /* Functions */

    my_function(11, 22);

    fn my_function(x: i32, y: i32) -> i32 {
        println!("The value of x is: {}", x);
        println!("The value of x is: {}", y);
        let sum = x + y;
        return sum;
    }

}
