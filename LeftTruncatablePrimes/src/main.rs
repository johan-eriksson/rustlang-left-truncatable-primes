mod prime;

use std::time::{Instant};
use std::thread;


fn main() {
    let start = Instant::now();
    run_sequentially();
    let seq_runtime = start.elapsed();
    run_in_parallel();
    let parallel_runtime = start.elapsed() - seq_runtime;
    println!("It took {0} s to run in sequence and {1} s to run in parallel", seq_runtime.as_secs_f64(), parallel_runtime.as_secs_f64());
}

fn run_in_parallel()
{
    let mut handles = Vec::new();
    for i in [2,3,5,7].iter()
{
    handles.push(thread::spawn(move || {
        let endpoints = find_all_endpoints(*i);
        println!("From thread: Starting point {0} has {1} end points", i, endpoints.len());
    }));
}

for handle in handles  {
    handle.join().unwrap();
}
    
}

fn run_sequentially()
{
    for i in [2,3,5,7].iter()
    {
        let endpoints = find_all_endpoints(*i);
        println!("Starting point {0} has {1} end points", i, endpoints.len());
    }
}

fn find_all_endpoints(starting_point:u128) -> Vec<u128>
{
    let mut ans = vec![starting_point];
    for n in left_concat_primes(starting_point)
    {
        ans.extend(find_all_endpoints(n));
    }

    return ans;
}

fn left_concat(num:u128, concat:u128) -> u128
{
    return num + concat*10_u128.pow((num as f64).log10().ceil() as u32);
}

fn left_concat_primes(p:u128) -> Vec<u128>
{
    let mut ans = Vec::new();
    for i in 1..10 {
        let candidate = left_concat(p, i);
        if prime::is_prime(candidate)
        {
            ans.push(candidate);
        }
    }

    return ans;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_left_concat_primes()
    {
        let expected = vec![17, 37, 47, 67, 97];
        let primes = left_concat_primes(7);
        assert_eq!(primes, expected)
    }

    #[test]
    fn test_left_concat_primes_2()
    {
        let expected = vec![269, 569, 769];
        let primes = left_concat_primes(69);
        assert_eq!(primes, expected)
    }

    #[test]
    fn test_find_endpoint()
    {
        let expected = 3947 as u128;
        let endpoint = *find_all_endpoints(3947).first().unwrap();
        assert_eq!(endpoint, expected);
    }
}
