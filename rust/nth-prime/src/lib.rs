pub fn nth(n: u32) -> Option<u32> {
    if n<=0 {
        None
    }else{
        let mut prime:Vec<u32> = Vec::with_capacity(128);
        prime.push(2);prime.push(3);prime.push(5);
        // prime[0]=2;prime[1]=3;prime[2]=5;
        if n <= 3{
            return Some(prime[(n-1) as usize]);
        }else{
            // let r = n-3
            let mut r=0;
            for i in 0..(n-3){
                r = prime[(2+i) as usize]+2;
                loop {
                    let factor_limit = (r as f64).sqrt() as u32;
                    if prime.iter().filter(|&&r|r<=factor_limit).any(|x| r%x == 0) {
                        r = r+2;
                        continue
                    }else{
                        break;
                    }
                }
                prime.push(r)
            }
            Some(r)
        }

    }
}
    
