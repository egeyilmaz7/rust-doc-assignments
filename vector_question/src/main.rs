use std::{collections::HashMap, vec};

fn main() {
    /*
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and
    mode (the value that occurs most often; a hash map will be helpful here) of the list.
    */

    let v = vec![1, 2, 2, 3, 2, 4, 2, 1];

    // sort v and take the length
    let mut sorted_v = v.clone();
    sorted_v.sort();
    let len = sorted_v.len();

    // find median in two options (even = average of two middle numbers, odd = middle number)
    let median = if len % 2 == 0 {
        (sorted_v[len / 2 - 1] + sorted_v[len / 2]) / 2
    } else {
        sorted_v[len / 2]
    };

    println!("Your median is: {}", median);
    println!("Your node is: {}", find_mode(&v))
}

fn find_mode(v: &Vec<i32>) -> i32 {
    let mut frequency = HashMap::new();

    // inside in the for loop we are going to use a hashmap.entry method this method need actual integers as argument becuase of that we are going to use &num instead of num
    for &num in v {
        *frequency.entry(num).or_insert(0) += 1; // or_insert method returns a reference we need to deref it thats why we used the *
    }

    // at this point, frequency looks like: {1: 2, 2: 4, 3: 1, 4: 1}

    // we are going to use a advanced concept from(chapter 13) iteretors and closures
    // let closure_name = |param1,param2| expression;
    // instead of createing a new function for small things we can use closures and make a anonymous functions

    frequency
        .into_iter() // go through the hashtable and make it a list of pairs
        .max_by_key(|(_, count)| *count) // we are going to find the list with the biggest count value
        .map(|(num, _)| num) // extract the value from (value,biggest count)
        .unwrap_or(0) // error handling

    /*
    {1: 2, 2: 4, 3: 1}
        ↓ into_iter()
    [(1,2), (2,4), (3,1)]
        ↓ max_by_key()
    (2, 4) ← biggest count
        ↓ map()
    2 ← just the number
        ↓ unwrap_or()
    2 ← final answer
     */
}
