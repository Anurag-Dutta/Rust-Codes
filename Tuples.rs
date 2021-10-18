fn main ()
{
    let t:(char,char,char,char,char,char,i32) = ('M','O','N','D','A','Y',1);
    let (A,B,C,D,E,F,G) = t;
    println!("The value present in the tuple are {}, {}, {}, {}, {}, {} and {}",A,B,C,D,E,F,G);
    let tuple = ("Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday");
    let (A,B,C,D,E,F,G) = tuple;
    println!("The value present in the tuple are {}, {}, {}, {}, {}, {} and {}",A,B,C,D,E,F,G);
}
