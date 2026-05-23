use std::io; //for just inputting something (why don't they have inbuilt stuff??)
use rand::Rng; //for random
use std::cmp::Ordering; //for that comparison
fn main()
{
    //TODO: Find out what macro means
    println!("Guess the number!"); 
    let secret_number=rand::thread_rng().gen_range(1..=100); //rand::thread_rng is just math.random(), gen_range gives the range. =100 means inclusive.
    println!("The secret number is {secret_number}");
    let mut counter:u32=1;
    loop {
        println!("Enter your number:"); 
        let mut guess=String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        println!("You guessed: {}", guess);
        let guess:u32=match guess.trim().parse() { //use match to use ok, err
            Ok(num) => num, //num just means int
            Err(_) => {println!("Please input a number"); continue;}, // _ is catch-all
        };
        match guess.cmp(&secret_number)
        {
            Ordering::Equal => {println!("You win! \n Loops taken: {}", counter); break;},
            Ordering::Greater => {println!("Input a smaller number! \n Current counter: {counter}"); counter+=1;},
            Ordering::Less => {println!("Input a bigger number! \n Current counter: {counter}"); counter+=1;},
        }
    }
}
