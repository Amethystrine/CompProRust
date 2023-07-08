fn solve() {
    input! {
        n:usize,
        a:String,
    }
    let v=String::from("atcoder");
    const MOD:i64 = 1000000007;
    let mut dp=vec![vec![0;8];(n+1)];
    dp[0][0]=1;
    for i in 0..n{
        for j in 0..8{
            dp[i+1][j]+=dp[i][j];
            dp[i+1][j]%=MOD;
            if j < 7 && v[j..j+1]==a[i..i+1]{
                dp[i+1][j+1]+=dp[i][j];
                dp[i+1][j+1]%=MOD;
            }
        }
    }
    println!("{}",dp[n][7]);
}
