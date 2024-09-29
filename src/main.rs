mod leetcode;
fn main() {
    // println!("Hello, world!");
    // let word = " ".to_string();

    // let result = leetcode::is_palindrome(word.clone());

    // let nums = vec![1, 2, 2];
    // let k = 3;
    let result = leetcode::min_sub_array_len(4, [1,4,4].to_vec());

    println!("The this is happening: {:?}", result)
}


