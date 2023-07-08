use proconio::input;
use std::cmp;

fn solve() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let mut ans=0.0;
    const PTD : f64 = 180.0/std::f64::consts::PI;
    for i in 0..n{
        let mut deg=vec![];
        for j in 0..n{
            if i==j{
                continue;
            }
            let dy=(xy[j].1-xy[i].1) as f64;
            deg.push(dy.atan2((xy[j].0-xy[i].0)as f64) * PTD);
        }
        deg.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut r=1;
        for l in 0..n-1{
            loop{
                let mut dd=deg[r]-deg[l];
                if dd < 0.0{
                    dd+=360.0;
                }
                if l==r{
                    break;
                }
                if dd > 180.0{
                    if ans < 360.0-dd{
                        ans=360.0-dd;
                    }
                    break;
                }
                ans=ans.max(dd);
                r+=1;
                if r>=n-1{
                    r=0;
                }
            }            
        }
    }
    println!("{:.10}",ans);
}
