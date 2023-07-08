use proconio::input;

fn solve(){
    input!{
        h:usize,
        w:usize,
        a:[[i32;w];h]
    }
    let mut sh=vec![0;h];
    let mut sw=vec![0;w];    
    for i in 0..h{
        for j in 0..w{
            sh[i]+=a[i][j];
            sw[j]+=a[i][j];
        }
    }
    for i in 0..h{
        for j in 0..w{
            print!("{} ",sh[i]+sw[j]-a[i][j]);
        }
        println!();
    }
}