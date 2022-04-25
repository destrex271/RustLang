fn main(){
    let tup = (500, 0.4, 1);
    println!("x: {}, y: {}, z: {}",tup.0,tup.1,tup.2);
    let y  = [6;8];
    println!("For Loop");
    for elem in y{
        println!("{}",elem);
    }
    println!("While Loop");
    let mut i = 0;
    while i < y.len(){
        println!("{}",y[i]);
        i+=1;
    }
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    println!("Countdown Method:");
    for number in (1..=4).rev(){
        println!("{}",number);
    }

    let mut str = String::from("hello");
    str.push_str("Hello");
    println!("{}",str);

}