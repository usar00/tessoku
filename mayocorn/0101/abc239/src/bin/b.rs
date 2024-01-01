use proconio::input;
fn main() {
    input! {
        x : i64
    }
    let result = if x < 0 && x % 10 != 0
    {
        x/10-1
    }else{
        x/10
    };
    println!("{}", result);
}
