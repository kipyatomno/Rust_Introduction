 use std::io;
 fn main (){
    let mut name = String::new();
    let mut age  = String::new();
    let mut course = String::new();
    let mut registration_number = String::new();
    println!("Whats your name");
    io::stdin().read_line (&mut name).expect ("Read Line Failed");
    println!("Whats your age");
    io::stdin().read_line (&mut age).expect ("Read Line Failed");
    println!("Whats your course");
    io::stdin().read_line (&mut course).expect ("Read Line Failed");
    println!("Whats your registration_number");
    io::stdin().read_line (&mut registration_number).expect ("Read Line Failed");
    println!("Whats your registration_number");
    println!("Your name is{name}");
    println!("Your age is {age}");
    println!("Your course is {course}");
    println!("Your registration_number is{registration_number}");


}

