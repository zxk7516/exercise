pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        None
    }else{
        let mut r = 0;
        let mut n_copy = n;
        loop {
            if n_copy == 1 {
                break;
            }
            if n_copy %2 == 0 {
                n_copy = n_copy /2;
            }else{
                n_copy = n_copy*3+1;
            }
            r = r +1;
        }


        Some(r)
    }

}
