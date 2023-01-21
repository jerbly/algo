/*
Create a data structure with an array [0u64; 1024]. This is a bit vector which has 65,536 elements, all of them zero (initially).
1. Set the i^th bit to 1 if and only if i is prime.
2. Add a method fn is_prime(n: usize) -> Option<bool>. If n >= 65536 you may return None.
   Otherwise, return true/false whether n is prime or not. You may not use divison, mod or multiplication in your solution.
3. Count the number of primes < 65536 using your data structure.
4. Write a routine that extracts n bits starting at bit m into a u64. You may assume that n<=64 and n>0 and m+n<=65536.
   You may not assume anything else about n and m.
*/
struct PrimeBits {
    primes: [u64; 1024],
}

impl PrimeBits {
    const MAXBITS: usize = 65536;
    const SQRT_MAXBITS: usize = 256;

    fn bit_div_mod64(n: u64) -> (u64, u64) {
        let q = n >> 6;
        let r = n & 0x3F;
        (q, r)
    }

    fn clear(&mut self, n: usize) {
        let (elem, bit) = Self::bit_div_mod64(n as u64);
        self.primes[elem as usize] &= !(1 << bit);
    }

    fn is_set(&self, n: usize) -> bool {
        let (elem, bit) = Self::bit_div_mod64(n as u64);
        let mask: u64 = 1 << bit;
        self.primes[elem as usize] & mask == mask
    }

    /// Return true/false whether n is prime or not.
    // You may not use divison, mod or multiplication in your solution.
    fn is_prime(&self, n: usize) -> Option<bool> {
        if n >= Self::MAXBITS {
            None
        } else {
            Some(self.is_set(n))
        }
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
    fn extract(&self, m: u64, n: u64) -> u64 {
        let (elem1, bit1) = Self::bit_div_mod64(m);
        let (elem2, bit2) = Self::bit_div_mod64(m + n);
        if n == 64 && bit1 == 0 {
            return self.primes[elem1 as usize];
        }
        if bit2 == 0 {
            return self.primes[elem1 as usize] >> (64 - n);
        }
        if elem1 == elem2 {
            // section from a single element
            let mut result = self.primes[elem1 as usize];
            result <<= 64 - bit2;
            return result >> (64 - n);
        }
        // section split across two element
        let mut back = self.primes[elem1 as usize];
        back >>= bit1;
        let mut result = self.primes[elem2 as usize];
        result <<= 64 - bit2;
        result >>= 64 - n;
        result | back
    }

    /// Sieve of Eratosthenes: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
    fn new() -> Self {
        let mut p = PrimeBits {
            primes: [0xFFFF_FFFF_FFFF_FFFFu64; 1024],
        };
        p.clear(0);
        p.clear(1);
        for i in 2..Self::SQRT_MAXBITS {
            if p.is_set(i) {
                let mut j = i * 2;
                while j < Self::MAXBITS {
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
        assert_eq!(p.num_primes(), 6542);
        assert_eq!(p.extract(60, 8), 0b1000_0010);
        assert_eq!(p.extract(0, 64), p.primes[0]);
        assert_eq!(p.extract(64, 64), p.primes[1]);
        // println!("{:064b}", p.primes[0]);
        // println!("{:064b}", p.primes[1]);
        // println!("{:064b}", p.extract(60, 64));
    }

    #[test]
    fn test2() {
        let p = PrimeBits::new();
        for i in 0..128 {
            for j in 1..=64 {
                println!("{:2},{:2} {:064b}", i, j, p.extract(i, j));
            }
        }
    }
}
