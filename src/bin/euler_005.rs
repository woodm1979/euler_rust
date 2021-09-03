//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::convert::TryInto;

use euler_rust::prime_lib::prime_factors;

fn main() -> Result<(), &'static str> {
    let ans = smallest_divisible(20)?;
    println!("{}", ans);
    Ok(())
}

fn smallest_divisible(max_divisor: u64) -> Result<u64, &'static str> {
    let mut cumulative_factors: HashMap<u64, u32> = HashMap::new();
    if max_divisor == 0 {
        return Err("There are no divisors for 0");
    }
    if max_divisor == 1 {
        return Ok(1);
    }

    for i in 2..=max_divisor {
        let prime_factors = prime_factors(&i)?;
        for (key, group) in (&prime_factors).into_iter().group_by(|&n| n).into_iter() {
            let count: u32 = group.count().try_into().unwrap();
            let mut key_max_count = count;
            if cumulative_factors.contains_key(key) {
                key_max_count = max(
                    key_max_count,
                    (*cumulative_factors.get(key).unwrap()).into(),
                );
            }
            cumulative_factors.insert(*key, key_max_count);
        }

        // println!("{} : {:?}", i, prime_factors);
    }
    // println!("cumulative_factors {:?}", cumulative_factors);

    let mut cum_value = 1;
    for (key, value) in cumulative_factors.iter() {
        cum_value *= key.pow(*value);
    }
    return Ok(cum_value);
}

#[cfg(test)]
mod tests {
    use crate::smallest_divisible;

    #[test]
    fn max_is_1() -> Result<(), String> {
        assert_eq!(smallest_divisible(1)?, 1);
        Ok(())
    }

    #[test]
    fn max_is_2() -> Result<(), String> {
        assert_eq!(smallest_divisible(2)?, 2);
        Ok(())
    }

    #[test]
    fn max_is_3() -> Result<(), String> {
        assert_eq!(smallest_divisible(3)?, 6);
        Ok(())
    }

    #[test]
    fn max_is_4() -> Result<(), String> {
        assert_eq!(smallest_divisible(4)?, 12);
        Ok(())
    }

    #[test]
    fn max_is_10() -> Result<(), String> {
        assert_eq!(smallest_divisible(10)?, 2520);
        Ok(())
    }
}
