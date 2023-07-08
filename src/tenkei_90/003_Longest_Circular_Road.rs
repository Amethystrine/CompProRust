use proconio::input;

fn dfs(now:usize, parent:usize, g : &Vec<Vec<usize>>) -> (usize,usize){
    let mut gv=now;
    let mut gd=0;
    for v in &g[now]{
        if *v==parent{
            continue
        }
        let res=dfs(*v,now,g);
        if gd<res.0 + 1{
            gd=res.0 + 1;
            gv=res.1;
        }
    }
    (gd,gv)
}

fn solve(){
    input!{
        n:usize,
    }
    let mut g=vec![vec![];n];
    for _ in 0..n-1{
        input!{
            mut a:usize,
            mut b:usize,
        }
        a-=1;
        b-=1;
        g[a].push(b);
        g[b].push(a);
    }
    let res1=dfs(0,n,&g);
    let res=dfs(res1.1,n,&g);
    println!("{}",res.0 + 1);
}
