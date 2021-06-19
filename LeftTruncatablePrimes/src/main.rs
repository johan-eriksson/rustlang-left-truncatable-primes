mod prime;

fn main() {
    
    for i in [3,5,7].iter()
    {
        let endpoints = find_all_endpoints(*i);
        println!("Starting point {0} has {1} end points", i, endpoints.len());
    }
}

fn find_all_endpoints(starting_point:i64) -> Vec<i64>
{
    
    let mut ans = vec![starting_point];
    if starting_point > 2_i64.pow(50)
    {
        return ans;
    }
    for n in left_concat_primes(starting_point)
    {
        ans.extend(find_all_endpoints(n));
    }

    return ans;
}

fn left_concat(num:i64, concat:i64) -> i64
{
    let concatted_string = format!("{}{}", concat.to_string(), num.to_string());
    let parsed = concatted_string.parse::<i64>();
    return parsed.unwrap();
    //return p + i*10_i64.pow((p as f64).log10().ceil() as u32);
}

fn left_concat_primes(p:i64) -> Vec<i64>
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
}
