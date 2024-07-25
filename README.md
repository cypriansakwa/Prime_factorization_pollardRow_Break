This Rust program performs prime factorization of a given number using Pollard's Rho algorithm, trial division, and checks for small prime factors. The  program  uses the 'num-bigint' library for huge integers and 'num-traits' for numeric traits.

The program has various functions for factoring a number:
1. 'pollards_rho': Determines a number's non-trivial factor using Pollard's Rho algorithm.
2. 'trial_division': Searches for small prime factors up to a certain limit.
3. 'prime_factors': Uses Pollard's Rho and trial division to determine all prime factors of a number.
4. 'is_prime': Determines whether a number is prime.
5.'sqrt': Calculates the integer square root using the Newton method.

git clone https://github.com/cypriansakwa/Prime_factorization_pollardRow_Break
.git
cd Prime_factorization_pollardRow_Break
