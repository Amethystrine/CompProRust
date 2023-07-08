use proconio::input;

fn solve() {
    const m: i64 = 1000000007;
    input! {
        mut n:usize,
        b:usize,
        k:usize,
    }
    input! {
        c:[usize;k]
    }
    let mut dp: Vec<i64> = vec![0; b];
    let mut ans: Vec<i64> = vec![0; b];
    ans[0] = 1;
    for v in c {
        dp[v % b] += 1;
    }
    let mut base: i64 = 10;
    while n > 0 {
        let mut ndp: Vec<i64> = vec![0; b];
        if n % 2 == 1 {
            let mut nans: Vec<i64> = vec![0; b];
            for ia in 0..b {
                for id in 0..b {
                    nans[(ia * (base as usize) + id) % b] += ans[ia] * dp[id];
                    nans[(ia * (base as usize) + id) % b] %= m;
                }
            }
            ans = nans;
        }
        for ia in 0..b {
            for id in 0..b {
                ndp[(ia * (base as usize) + id) % b] += dp[ia] * dp[id];
                ndp[(ia * (base as usize) + id) % b] %= m;
            }
        }
        dp = ndp;
        n /= 2;
        base = (base * base) % (b as i64);
    }
    println!("{}", ans[0]);
}