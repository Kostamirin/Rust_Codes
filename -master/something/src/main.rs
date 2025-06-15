fn main() 
{
    print!("[");
    for i in 0..1000 + 1 {
        if i < 1000 {
            print!("{}, ",i);
        }
        else{
            print!("{}",i);
        }        
    }
    print!("]");
}
