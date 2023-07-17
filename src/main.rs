fn main() {
    let s1:String = "Hello".to_string();
    let s2:String = "World".to_string();
    //println!("{}",stringmul(s1,s2));
    //println!("{}",strng_format(s1,s2));
    razdeleniyachar(s1);
}
fn razdeleniyabyte(s:String){
    for b in s.bytes() {
        println!("{b}");
    }
}

fn razdeleniyachar(s:String){
    for ch in s.chars() {
        println!("{ch}");
    }
}

fn strokindex(){
    let s1:String = "LOLENG".to_string();
    println!("{}",&s1[..2]);
}

fn strng_format(s1:String,s2:String)->String{
    format!("{s1} {s2}")
}
fn stringmul(s1:String,s2:String)->String{
    s1+&s2
}

fn stringpush() {
    let mut s:String = "Hello".to_string();
    s.push_str(" World");
    s.push('!');//sadece push string veri turunun arkasina bir char eklemek icin kullanilir ve char verileri caft tirnak il degil tek tirnak ile eklenmaktedir.
    println!("{s}");
}