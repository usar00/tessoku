use proconio::input;
fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let mut ans = false;
    for i in a..b+1{
        if 100 % i == 0{
            ans = true;
            break;
        }
    }
    if ans{
        println!("Yes");
    }else{
        println!("No");
    }
}
