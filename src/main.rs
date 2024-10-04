mod leetcode;
fn main() {
    // println!("Hello, world!");
    // let word = " ".to_string();

    // let result = leetcode::is_palindrome(word.clone());

    // let nums = vec![1, 2, 2];
    // let k = 3;
    let result = leetcode::remove_duplicates(&mut [1,1,2].to_vec());

    println!("The this is happening: {:?}", result)
}


