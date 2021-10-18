fn main()
{
    let mut rows = 5;
    for i in 1..rows+1 
    {
        for j in 1..i+1
        {
            print!("* ");
        }
        println!("");
    }
}