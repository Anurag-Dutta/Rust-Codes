fn main()
{
    println!("5! = {}",fact(5));
}
fn fact(n:i32) ->i32 // Here the part after the -> represents the return type of the function.
{
    if n==1
    {
        1
    }
    else if n==0
    {
        1
    }
    else
    {
        n*fact(n-1)
    }
}