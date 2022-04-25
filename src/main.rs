fn main() {
    let s1: String = String::from("TestString!");
    // The Value will be moved from s1 to s2 and s1 would be dropped
    let s2: String = s1; // s2 borrowed the value of s1 and s1 was dropped out of scope to prevent double free error
    /*
    double free error occurs when two variables pointing to the same memory location are dropped off from their scope
    this leads to the system freeing the same memory location twice in a row which might lead to memory corruption
    and in turn generate security vulnerabilitites
    */
    // println!("{}",s1);
    println!("{}",s2);
    /*
    Since Rust never creates any deep copies of your data and in turn prefers "moving" the values from one
    variable to another or *the OwnerShip* of the value from one variable to another, therefore any kind of
    automatic copying can be assumed to be inexpensive in terms of runtime performance. 
    */

    /*
    But if cloning is required,in case of a String(and not a str, which is immutable), we can use the inbuilt
    clone function of String.
    */ 
    /*
    Cloning usually indicates that the given code is arbitary in nature and can be ieffecient as well as taxing
    for our system
    VISUAL INDICATOR that something different is going on. 
    */
}
