use std::mem;

#[derive(PartialEq)]
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

        if self.inner.len() <= 1 {
            return Some(self.inner.clone())
        }

        let mut result= vec![];
        let current = 0 as usize;

        while current < self.inner.len()  {
            if self.inner[current] == result[0] {
                result.push(self.inner[current])
            } else {
                self.inner.drain(0..current);
                break
            }
        }
        Some(result)
    }
}

// fn main() {
//     let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
//     // groups:     |->|---->|->|->|--->|----------->|--->|
//     assert_eq!(
// 	    Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
// 	    vec![
// 	        vec![4],
//     	    vec![1, 1],
// 	        vec![2],
//     	    vec![1],
// 	        vec![3, 3],
// 	        vec![-2, -2, -2],
//     	    vec![5, 5],
// 	    ]
//     );

//     let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
//     // groups:      |->|---->|---->|----|->|----->|->|
//     assert_eq!(
// 	    Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
// 	    vec![
// 	        vec![1],
//     	    vec![2, 2],
// 	        vec![1, 1],
// 	        vec![2, 2],
//     	    vec![3],
// 	        vec![4, 4],
// 	        vec![3],
// 	    ]
//     )
// }
