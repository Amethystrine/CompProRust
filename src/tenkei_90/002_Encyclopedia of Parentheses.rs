use proconio::input;


fn solve(){
    input!{
        n:i64,
    }
    let mut nb:i64=1<<n;
    for b in (0..nb){
        let mut c=0;
        let mut ok=true;
        let mut ans = String::from("");
        for i in (0..n).rev(){
            if (((b>>i)&1) != 0){
                c-=1;
                ok&=c>=0;
                ans.push(')');
            }else{
                c+=1;
                ans.push('(');
            }
        } 
        if(c==0 && ok){
            println!("{}",ans);
        }
    }
}
