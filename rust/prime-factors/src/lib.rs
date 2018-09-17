pub fn factors(n: u64) -> Vec<u64> {
    let mut results:Vec<u64>  = Vec::new();
    let mut computed = n;
    let mut prime:Vec<u64> = vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47];
    let mut cur_prime = prime[0];
    let mut cur_prime_index = 0;
    let mut max_prime_in_candicate = prime[prime.len()-1]+2;
    while computed > 1 {
        while computed % cur_prime == 0 {
            results.push(cur_prime);
            computed = computed / cur_prime;
        }
        if cur_prime_index<prime.len()-1 {
            cur_prime_index = cur_prime_index + 1;
            cur_prime = prime[cur_prime_index];
        }else{
            'outer: loop {
                let sqrt_max_prime_in_candicate = (max_prime_in_candicate as f64).sqrt() as u64;
                'inner: for i in 0..prime.len(){
                    if prime[i] > sqrt_max_prime_in_candicate {
                        prime.push(max_prime_in_candicate);
                        max_prime_in_candicate = max_prime_in_candicate + 2;
                        break 'outer;
                    }
                    if max_prime_in_candicate % prime[i] == 0 {
                        break 'inner;
                    }
                    

                }
                max_prime_in_candicate = max_prime_in_candicate + 2;
            }
        }

    }

    results
}
