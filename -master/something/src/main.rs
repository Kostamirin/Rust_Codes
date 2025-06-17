use log::error;

pub struct something;

impl something {
    pub fn calculation(first :i32, simbol :char, second :i32) -> f32 {
        if simbol == '+' {
            (first + second) as f32
        }
        else if simbol == '-' {
            (first - second) as f32
        }
        else if simbol == '*' {
            (first * second) as f32
        }
        else if simbol == '/' {
            if second != 0 {
                (first / second) as f32
            }
            else {
                println!("Division by zero");
                return 0.0;
            }
        }
        else{
            println!("Not a supported symbol");
            return 0.0;
        }
    }
}

fn main()
{
    let first = 156;
    let simbol = '/';
    let second = 8;
    
    let result = something::calculation(first, simbol, second);
    println!("Result: {}", result);
}
