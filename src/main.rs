use proconio::input;

fn solve() {
    input!{
        n:usize,
        cp:[(usize,usize);n],
        q:usize,
        lr:[(usize,usize);q],
    }
    let mut a1=vec![0;n+1];
    let mut a2=vec![0;n+1];
    for i in 0..n{
        a1[i+1]=a1[i];
        a2[i+1]=a2[i];
        if cp[i].0==1{
            a1[i+1]+=cp[i].1;
        }else{
            a2[i+1]+=cp[i].1;
        }
    }
    for i in 0..q{ 
        println!("{} {}",a1[lr[i].1]-a1[lr[i].0-1],a2[lr[i].1]-a2[lr[i].0-1]);
    }

}

fn main() {
    solve();
}
