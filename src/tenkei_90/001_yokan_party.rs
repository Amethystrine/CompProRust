use proconio::input;

fn solve(){
    input!{
        n:i64,
        l:i64,
        k:i64,
    }
    input!{
        a:[i64;n]
    }
    let mut ls:i64=0;
    let mut rs:i64=1000000001;
    while ls+1<rs {
        let m=(ls+rs)/2;
        let mut c=0;
        let mut prev=0;
        for v in &a {
            if *v-prev>=m {
                c+=1;
                prev=*v;
            }
        }
        if l-prev>=m{
            c+=1;
        }
        if c>=k+1 {
            ls=m;
        }else{
            rs=m;
        }
    }
    println!("{}",ls);
}
