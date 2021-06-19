fn is_prime(n: i64) -> bool
{
    if n <= 3
    {
        return n > 1;
    }
    if n % 2 == 0 || n % 3 == 0
    {
        return false;
    }
    
    let mut i = 5;
    while i*i <= n
    {
        if n % i == 0 || n % (i + 2) == 0
        {
            return false;
        }
        i += 6
    }
    
    return true;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_is_prime_with_primes()
    {
        let primes =  vec![2,3,5,7,127,524287,1000003];
        for prime in primes {
            assert_eq!(is_prime(prime), true);    
        }
    }

    #[test]
    fn test_is_prime_with_composites()
    {
        let composites =  vec![4,6,42,98,999,1698661];
        for composite in composites {
            assert_eq!(is_prime(composite), false);    
        }
    }
}