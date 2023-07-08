use proconio::input;
use std::cmp;

fn solve() {
    input! {
        n:usize,
        mut a:[i64;n],
        q:usize,
        b:[i64;q],
    }
    a.push(-1000000000000);
    a.push(1000000000000);
    a.sort();

    for v in b {
        let mut l = 0;
        let mut r = a.len();
        while l + 1 < r {
            let m = (l + r) / 2;
            if a[m]<v{
                l=m;
            }else {
                r=m;
            }
        }
        println!("{}",cmp::min(&a[r]-v,v-&a[l]));
    }
}
