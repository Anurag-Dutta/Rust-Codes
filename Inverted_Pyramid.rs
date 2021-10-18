fn main()
{
    let mut rows = 5;
    for i in 1..rows+1 
    {
        for i in (i..rows+1).rev() 
        {
            print!("* ");
        }
        println!("");
    }
}