use std::io;
//io is  library for user input/ output. std stands for standard library
use rand::Rng;
//Rng trait defines methods that the random num generator needs and it must be in scope to use.


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //rand::thread_rng gives us the random number were gonna use. 
    //local to the current thread(?) of execution and is seeding in the OS?
    //then call the gen range method to gen a random number within the specified range. 

    println!("The secret number is: {secret_number}");

    println!("please input your guess");
    //i dont understand the macro ! 

    let mut guess = String::new(); //mut makes a varibale mutabe - rust vars are immutible on init.
    //guess is being bound to the result of a string::new which is a fn that returns a new instance of a string

    //the :: situation indicates that /new/ is an associated fn of the String type. associated fn is a fn implemented on a Type.
    //teh new fn creates a new empty string.

    io::stdin()
        .read_line(&mut guess)
        //read_line is a method of stdin (standard input)
        //you pass &mut guess as the arg to tell the var what string to store user input in (in guess)

        //The full job of read_line is to take whatever the user types into standard input and append 
        //that into a string (without overwriting its contents), so we therefore pass that string as an argument.
        // The string argument needs to be mutable so the method can change the string’s content.

        //the & symbol indicates that this is a reference - sort of like @api in LWC. multiple parts
        //of your code can now access this data without needing to copy it in memory multiple times. - refs are immutable.
        .expect("failed to read line");
        //read_line returns a result - this is an enumeration or an enum. this is a type that can be in one of multiple possible states.
        //each possible state is a variant.
        //the result type is like a promise in js. results have 'ok' and 'err' 
        //Result has methods like .expect you can call. //you need to write error handling code or youll get a warning bc your program will crash. 
        println!("You guessed: {guess}")
        //prints the value stored in mut guess using simpler template literal or sort of jsx.
}
