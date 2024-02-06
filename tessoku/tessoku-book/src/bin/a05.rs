use proconio::input;
fn main() {
    input! {
        n: i32,
        k: i32,
    }
    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            let sum_k = k - i - j;
            if (sum_k >= 1) && (sum_k <= n) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
