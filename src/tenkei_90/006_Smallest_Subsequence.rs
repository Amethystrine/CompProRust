use proconio::input;

fn solve() {
    input!{
        n:usize,
        k:usize,
        s:String
    }
    let mut ans=String::from("");
    let mut pos=vec![vec![];26];
    let sb=s.as_bytes();
    for i in 0..n{
        pos[(sb[i]-b'a') as usize].push(i);
    }
    for i in 0..26{
        pos[i].push(n);
    }
    let mut ans=vec![];
    let mut now=0;
    for b in 0..k{
        for v in 0..26{
            let mut l=-1;
            let mut r=pos[v].len() as i64;
            while l+1<r{
                let m=(l+r)/2;
                if pos[v][m as usize] < now{
                    l=m;
                }else{
                    r=m;
                }
            }
            let npos=r as usize;
            if pos[v][npos]+k-b <= n{
                ans.push((v as u8)+b'a');
                now=pos[v][npos]+1;
                break;
            }
        }
    }
    println!("{}",String::from_utf8(ans).unwrap());
}