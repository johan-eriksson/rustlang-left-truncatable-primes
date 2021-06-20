use std::u128;

pub(crate)

use ramp::Int;

pub fn is_prime(n: u128) -> bool {
    let n_str = n.to_string();
    let mut witnesses = n_str.len();

    if witnesses < 3 {
        witnesses = 3;
    }

    is_prime_with_witnesses(&n_str, witnesses)
}


pub fn is_prime_with_witnesses(n_str: &str, witnesses: usize) -> bool {
    let two        = Int::from_str_radix("2", 10).unwrap();
    let n          = Int::from_str_radix(n_str, 10).unwrap();
    let n_sub: Int = n.clone() - 1;

    if n == 2 || n == 3 || n == 5 {
       return true;
    }

    if n < 2 || (n.divmod(&two).1 == 0) {
       return false;
    }

    let mut exponent = n_sub.clone();
    let mut trials   = Int::zero();

    while exponent.divmod(&two).1 == 0 {
        exponent = exponent / 2;
        trials   = trials + 1;
    }

    'LOOP: for i in 1..(witnesses + 1) {
        let mut result = powmod(&(two.clone() + i), &exponent, &n);

        if result == 1 || result == n_sub {
            continue;
        }

        let mut verified = 1;

        while verified < trials {
            result = result.square().divmod(&n).1;

            if result == 1 {
                return false;
            }

            if result == n_sub {
                continue 'LOOP;
            }

            verified = verified + 1;
        }

        return false;
    }

    return true
}

fn powmod(base: &Int, exponent: &Int, modulus: &Int) -> Int {
    if *base == Int::zero() {
        return match *exponent == Int::zero() {
            true  => Int::one(),
            false => Int::zero(),
        }
    }

    if *modulus == Int::one() {
        return Int::zero();
    }

    let exponent_in_binary      = Int::to_str_radix(&exponent, 2, false);
    let mut exponent_chars_revd = exponent_in_binary.chars().rev();
    let mut exponent_length     = exponent_in_binary.len();
    let mut my_base             = base.clone();
    let mut result              = Int::one();

    while exponent_length > 0 {
        exponent_length = exponent_length - 1;

        if exponent_chars_revd.next().unwrap() == '1' {
            result = result * my_base.clone();
            result = result.divmod(&modulus).1;
        }

        my_base = my_base.square();
        my_base = my_base.divmod(&modulus).1;
    }

    result
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