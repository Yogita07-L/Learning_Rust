fn main() {
    println!("************Ownership************");

    /* "hello" is a STRING LITERAL. 
        It's hard-coded in the binary. */
    fn a(){
        let _x = "hello";
        let _y = 22;
        b();
    }
    /* String::from creates a heap-allocated String from a literal.
        Unlike the literal itself, this 'String' can be mutated and grown. */   
    fn b(){
        let _x = String::from("world");
    }

    /* Ownership Rules -
       1. Each value in Rust has a variable that’s called its owner.
       2. There can only be one owner at a time.
       3. When the owner goes out of scope, the value will be dropped (deallocated). */
    

    let x = 5;
    let y = x; /*Copy */

    let s1 = String::from("hello");
    //let _s2 = s1; /* s1 is MOVED to s2(not shallow copy). s1 is now INVALID. */
    //println!("{}", s1); /* ERROR! Borrow checker says s1 is "moved" */  

    let _s3 = s1.clone(); /*Clone() is used to deeply copy the heap data of the String, not just the stack data*/

    /* Rules of References in Rust-
    1. At any given time, you can have either one mutable reference or any number of immutable references.
    2. References must always be valid. */
    let mut s = String::from("Reference");

    let r1 = &s;
    let r2 = &s;

    println!("{},{}",r1, r2);

    let r3 = &mut s;
    println!("{}", r3);






}
