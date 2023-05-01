// ============================================================================
//
// problem_2_8 - Solution to the Problem 2.8
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
use rand::{thread_rng, Rng};


// ============================================================================
// Main function section
// ============================================================================

fn main() {
    let mut arr: [i16; 30] = [0; 30];
    thread_rng().fill(&mut arr[..]);
    let numbers: Vec<i16> = Vec::from(arr);

    let (l1, l2) = max2(&numbers);

    println!("a({})={}, a({})={}", l1, numbers[l1], l2, numbers[l2]);

    let mut k: usize = 0;
    while numbers.len() > k {
        println!("{}", numbers[k]);

        k = k + 1;
    }
}


// ============================================================================
// User defined functions section
// ============================================================================

// ----------------------------------------------------------------------------
//
// # Function "max2"
//
// Returns a tuple containing the indices of the two maximum values in
// the input vector.
//
// The `max2` function takes a reference to a vector of `i16` integers and
// returns a tuple of two 'usize` values. The function is used to find the
// indices of the two maximum values in the input vector.
//
// # Arguments
//
// * `a` - A reference to a vector of `i16` integers.
//
// # Returns
//
// A tuple containing the indices of the two maximum values in the input vector.
// The first value in the tuple is the index of the maximum value, and the
// second value is the index of the next maximum value. If there are multiple
// occurrences of the maximum value, the function returns the indices of the
// first two occurrences.
//
// # Example
//
// ```
// let v = vec![1, 3, 2, 3, 4, 5];
// let (max_index1, max_index2) = max2(&v);
// assert_eq!(max_index1, 5);
// assert_eq!(max_index2, 4);
// ```
//
// ----------------------------------------------------------------------------
fn max2(a: &Vec<i16>) -> (usize, usize) {
    let mut k:   usize = 0;
    let mut l1:  usize = 0;
    let mut l2:  usize = 0;
    let mut max: &i16 = &a[0];

    if !a.is_empty() {
        while a.len() > k {
            if max < &a[k] {
                max = &a[k];
                l2 = l1;
                l1 = k;
            }

            k = k + 1;
        }
    }

    (l1, l2)
}

