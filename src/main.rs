
// Moving only occurs with data types that use heap storage
// Stack stroage data types do not follow the concept of wonership


fn main() {
    let s1: String = String::from("TestString!");
    // The Value will be moved from s1 to s2 and s1 would be dropped
    let mut s2: String = s1; // s2 borrowed the value of s1 and s1 was dropped out of scope to prevent double free error
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

    // get_ownership(s2); passing ownership to function
    s2 = get_return_ownership(s2); // Passes ownershp to function and then return s it back to s2
    // println!("{}",s2) <= Throws the borrow error as ownership was passed to the function
    // let s2: String = String::from("Akshat");
    println!("{}",s2);


    // Function returning info as a tuple
    // let (x, len) = gen_string_info(s2);

    // println!("{} has a length of {}",x,len);
    println!("{} length from reference", gen_string_info_via_ref(&s2));
    add_a(&mut s2);
    println!("{}",s2);
    loop_thru_string(&s2);

}


// fn get_ownership(x:String){
//     println!("{}",x);
// }

fn get_return_ownership(x:String) -> String{
    return x;
}

fn gen_string_info(x:String) -> (String, usize){
    let length = x.len();
    return (x, length)
}

fn gen_string_info_via_ref(x: &String) -> usize{
    return x.len();
}

fn add_a(st : &mut String){
    st.push_str("A")
}

fn loop_thru_string(st : &String){
    let bytes = st.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        println!("{} at {}", String::from_utf8_lossy(&[item]), i);
    }
}

/*
 * At any given time, you can have either one mutable reference or any number of immutable references.
 * References must always be valid.
 */


