use std::{iter::once, path::Display};

mod leetcode;
// mod implitts;

// fn main() {
//     // println!("Hello, world!");
//     // let word = " ".to_string();

//     // let result = leetcode::is_palindrome(word.clone());

//     // let nums = vec![1, 2, 2];
//     // let k = 3;
//     let result = leetcode::subarrays_with_k_distinct([1,2,1,3,4].to_vec(), 3);

//     println!("The this is happening: {:?}", result)
// }

struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq + Copy> Iterator for Groups<T> {
    type Item = Vec<T>;

    // TODO: Write the rest of this implementation.
    fn next(&mut self) -> Option<Self::Item> {
        // Take the first item from the iterator (or return None if empty)
        let mut iter_self_gp = self.inner.clone().into_iter();
        let first = iter_self_gp.next()?;

        // Start a new group with the first item
        let mut group = vec![first];

        // Collect all consecutive identical items into the group
        while let Some(next_item) = iter_self_gp.next() {
            println!("hey 24>>: {:?}", self.inner.len());
            if next_item == group[0] {
                group.push(next_item);
                continue;
            } else {
                // If we encounter a different item, put it back into the iterator
                println!("hey 23------->>: {:?}", iter_self_gp.len());
                self.inner = std::iter::once(next_item).chain(iter_self_gp).collect::<Vec<_>>();
                break;
            }
        }

        // Return the collected group
        Some(group)
    }
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    )
}



