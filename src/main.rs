use std::io;

fn main()
{
    
    println!("Input Fahrenheit temp:");
     loop
    {
        let mut _input: String = String::new();
        let mut _result: f32;

        io::stdin()
            .read_line(&mut _input)
            .expect("Input a number or type exit");

        if _input == "exit"
        {
            break;
        } else 
        {
        let _input: f32 = _input.trim().parse().expect("deez"); // 10/10 debugging method
        let _result: f32 = (_input - 32.0) * 1.8;
        let _result = _result.floor();

        println!("celsius temp is {_result}");
        }
        
    }

    println!("good bye");
    
}