use std::io;

fn main()
{
    let mut input: String; //fahrenheit input
    let mut result: u32 = 0;

    println!("Input Fahrenheit temp:");
    while input != "exit"
    {
        io::stdin()
            .read_line(&mut input)
            .expect("Input a number or type exit");
        
        let input: u32 = input.trim().parse().expect("deez");

        let result: u32 = ((32 * input) * 5/9);

        println!("celsius temp is {result}");
    }

    println!("good bye");
    
}