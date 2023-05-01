// ============================================================================
//
// problem_2_9 - Solution to the Problem 2.9
//
//  Copyright (C) 2023 Ljubomir Kurij <ljubomir_kurij@protonmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// ============================================================================


// ============================================================================
//
// 2023-05-01 Ljubomir Kurij <ljubomir_kurij@protonmail.com>
//
// * main.rs: created.
//
// ============================================================================


// ============================================================================
// Used libraries section
// ============================================================================


// ============================================================================
// Main function section
// ============================================================================

fn main() {
    let primes: Vec<u32> = primes_up_to(100);

    let mut k: usize = 0;
    while primes.len() > k {
        println!("{}", primes[k]);

        k = k + 1;
    }
}


// ============================================================================
// User defined functions section
// ============================================================================

// ----------------------------------------------------------------------------
//
// # Function "removel"
//
// Removes all occurrences of a given element from the end of the input vector.
//
// The `removel` function takes a `u32` element and a mutable reference to
// a vector of `u32` integers. The function removes all occurrences of
// the given element from the end of the vector, in reverse order.
//
// # Arguments
//
// * `se` - The `u32` element to remove from the vector.
// * `a` - A mutable reference to a vector of `u32` integers.
//
// # Example
//
// ```
// let mut v = vec![1, 2, 3, 2, 4, 2, 5];
// removel(2, &mut v);
// assert_eq!(v, vec![1, 3, 4, 5]);
// ```
//
// # Notes
//
// The function modifies the input vector in place, and does not return
// a value. If the input vector is empty, the function has no effect.
//
// ----------------------------------------------------------------------------
fn removel(se: u32, a: &mut Vec<u32>) {
    let n: usize = a.len();
    let mut k: usize = 0;

    if !a.is_empty() {
        while n > k {
            let idx: usize = n - (k + 1);
            if se == a[idx] {
                a.remove(idx);
            }

            k = k + 1;
        }
    }
}


// ----------------------------------------------------------------------------
//
// # Function "primes_up_to"
//
// Returns a vector of all prime numbers up to a given limit.
//
// The `primes_up_to` function takes a `usize` limit and returns a vector of
// `u32` prime numbers up to given limit. The function uses the Sieve of
// Eratosthenes algorithm to generate the prime numbers.
//
// # Arguments
//
// * `m` - The limit up to which to generate prime numbers.
//
// # Returns
//
// A vector of all prime numbers up to the given limit, in ascending order.
// The vector is empty if the limit is less than or equal to 2.
//
// # Example
//
// ```
// let primes = primes_up_to(20);
// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
// ```
//
// # Notes
//
// The function initializes a vector of `u32` integers to contain all numbers
// from 1 up to the limit `m`. It then uses the Sieve of Eratosthenes
// algorithm to generate the prime numbers.
//
// The algorithm proceeds as follows:
//
// 1. Starting from the first prime number (2), mark all its multiples
//    (4, 6, 8, etc.) as composite numbers by setting them to 1 in the vector.
// 2. Move to the next unmarked number, which is a prime, and repeat step 1.
// 3. Repeat step 2 until all numbers up to the limit have been checked.
//
// After the algorithm finishes, the vector will contain all prime numbers
// up to the given limit.
//
// The function modifies the vector in place to mark composite numbers, using
// the `removel` function to remove the number 1 from the final result. If the
// limit is less than or equal to 2, the vector will be empty.
//
// ----------------------------------------------------------------------------
fn primes_up_to(m: usize) -> Vec<u32> {
    let mut k: usize = 0;
    let mut primes: Vec<u32> = Vec::new();

    if 2 < m {
        while m > k {
            primes.push((k + 1) as u32);

            k = k + 1;
        }
    }

    k = 1;
    while m > usize::pow(k + 1, 2) {
        if 1 != primes[k] {
            let mut j: usize = 2*k + 1;

            while primes.len() > j {
                primes[j] = 1;

                j = j + (k + 1);
            }
        }

        k = k + 1;
    }

    removel(1, &mut primes);

    primes
}
