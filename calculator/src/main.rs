use std::io;

fn main() {
    loop {
        println!("Press 1 for add");
        println!("Press 2 for sub");
        println!("Press 3 for product");
        println!("Press 4 for division");
        println!("Press 5 for exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to take input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice==5{
            break;
        }

        let mut num1 = String::new();
        let mut num2 = String::new();

        println!("Please Enter number 1");
        io::stdin().read_line(&mut num1).expect("Failed to take input");
        println!("Please Enter number 2");
        io::stdin().read_line(&mut num2).expect("Failed to take input");

        let num1:u32 = match num1.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        };
        let num2:u32 = match num2.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        };

        match choice {
            1 =>{
                let res = num1+num2;
                println!("The Addition of {} and {} is {}",num1,num2,res);
            }
            2=>{
                let res = num1-num2;
                println!("The Subtraction of {} and {} is {}",num1,num2,res);
            }
            3=>{
                let res = num1*num2;
                println!("The Product of {} and {} is {}",num1,num2,res);
            }
            4=>{
                let quo = num1/num2;
                let rem = num1%num2;
                println!("The Quotient of {} and {} is {} and remainder is {}",num1,num2,quo,rem);
            }
            _=>{
                println!("Invalid Choice")
            }
        }
    }
}
