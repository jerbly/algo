/*
Create a data structure with an array [0u64; 1024]. This is a bit vector which has 65,536 elements, all of them zero (initially).
1. Set the i^th bit to 1 if and only if i is prime.
2. Add a method fn is_prime(n: usize) -> Option<bool>. If n >= 65536 you may return None.
   Otherwise, return true/false whether n is prime or not. You may not use divison, mod or multiplication in your solution.
3. Count the number of primes < 65536 using your data structure.
4. Write a routine that extracts n bits starting at bit m into a u64. You may assume that n<=64 and n>0 and m+n<=65536.
   You may not assume anything else about n and m.
*/
const MAXBITS: usize = 65536;
struct PrimeBits {
    primes: [u64; 1024],
}

impl PrimeBits {
    fn clear(&mut self, n: usize) {
        let elem: usize = n / 64;
        let bit: u64 = n as u64 % 64;
        self.primes[elem] &= !(1 << bit);
    }

    fn is_set(&self, n: usize) -> bool {
        let elem: usize = n / 64;
        let bit: u64 = n as u64 % 64;
        let mask: u64 = 1 << bit;
        self.primes[elem] & mask == mask
    }

    /// Return true/false whether n is prime or not.
    // You may not use divison, mod or multiplication in your solution.
    fn is_prime(&self, n: usize) -> Option<bool> {
        if n >= MAXBITS {
            None
        } else {
            let (elem, bit) = Self::bit_div(n as u64, 64);
            let mask: u64 = 1 << bit;
            Some(self.primes[elem as usize] & mask == mask)
        }
    }

    /// recursive bitwise division
    fn bit_div(dividend: u64, divisor: u64) -> (u64, u64) {
        Self::division(dividend, divisor, divisor, 0)
    }

    fn division(dividend: u64, mut divisor: u64, origdiv: u64, mut remainder: u64) -> (u64, u64) {
        println!(
            "Entered: dd={:08b},{:3} dv={:08b},{:3}",
            dividend, dividend, divisor, divisor
        );

        let mut quotient: u64 = 1;
        if dividend == divisor {
            return (1, 0);
        } else if dividend < divisor {
            return (0, dividend);
        }

        while divisor <= dividend {
            divisor <<= 1;
            quotient <<= 1;
        }

        if dividend < divisor {
            divisor >>= 1;
            quotient >>= 1;
        }

        println!(
            "Leaving: dd={:08b},{:3} dv={:08b},{:3} qu={:08b},{:3}",
            dividend, dividend, divisor, divisor, quotient, quotient
        );

        let (q, r) = Self::division(dividend - divisor, origdiv, origdiv, remainder);
        quotient += q;
        remainder = r;

        (quotient, remainder)
    }

    /// number of primes
    fn num_primes(&self) -> usize {
        let mut num = 0;
        for i in self.primes {
            num += i.count_ones();
        }
        num as usize
    }

    /// extracts n bits starting at bit m into a u64
    fn extract(&self, m: usize, n: usize) -> u64 {
        let mut result = 0u64;
        let mut i = 0;
        while i < n {
            if self.is_set(m + i) {
                result |= 1 << i;
            }
            i += 1;
        }
        result
    }

    /// Sieve of Eratosthenes: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
    fn new() -> Self {
        let mut p = PrimeBits {
            primes: [0xFFFF_FFFF_FFFF_FFFFu64; 1024],
        };
        p.clear(0);
        p.clear(1);
        for i in 2..MAXBITS {
            if p.is_set(i) {
                let mut j = i * 2;
                while j < MAXBITS {
                    p.clear(j);
                    j += i;
                }
            }
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = PrimeBits::new();
        assert_eq!(p.is_prime(199), Some(true));
        assert_eq!(p.is_prime(198), Some(false));
        println!("{}", p.num_primes());
        println!("{:064b} {:064b}", p.primes[0], p.primes[1]);
        println!("{:08b}", p.extract(60, 8));
    }
    #[test]
    fn test2() {
        let dividend = 195;
        let divisor = 7;

        let (q, r) = PrimeBits::bit_div(dividend, divisor);
        println!("{},{}", q, r);
        assert_eq!(q, dividend / divisor);
        assert_eq!(r, dividend % divisor);
    }
}
