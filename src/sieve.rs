/*
Create a data structure with an array [0u64; 1024]. This is a bit vector which has 65,536 elements, all of them zero (initially).
1. Set the i^th bit to 1 if and only if i is prime.
2. Add a method fn is_prime(n: usize) -> Option<bool>. If n >= 65536 you may return None.
   Otherwise, return true/false whether n is prime or not. You may not use divison, mod or multiplication in your solution.
3. Count the number of primes < 65536 using your data structure.
4. Write a routine that extracts n bits starting at bit m into a u64. You may assume that n<=64 and n>0 and m+n<=65536.
   You may not assume anything else about n and m.
*/

fn primes(n: usize) {
    let mut bools = vec![true; n];
    bools[0] = false;
    bools[1] = false;
    for i in 2..n {
        if bools[i] {
            let mut j = i * 2;
            while j < n {
                bools[j] = false;
                j += i;
            }
        }
    }
    for (i, b) in bools.iter().enumerate() {
        if *b {
            println!("{i}");
        }
    }
}

#[test]
fn test() {
    primes(15);
}
