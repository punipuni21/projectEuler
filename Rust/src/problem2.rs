fn main() {
    

    let mut a:i64 = 0;
    let mut b:i64 = 1;
    let mut upperLimit:i64 = 4000000;
    let mut ans:i64 = 0;
    while b < upperLimit{
        let tmp:i64 = a+b;
        if  tmp % 2 == 0{
            ans += tmp;
        }
        a = b;
        b = tmp;
    }
    println!("ans= {}",ans)
}
