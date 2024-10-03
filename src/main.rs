mod leetcode;
fn main() {
    // println!("Hello, world!");
    // let word = " ".to_string();

    // let result = leetcode::is_palindrome(word.clone());

    // let nums = vec![1, 2, 2];
    // let k = 3;
    let result = leetcode::max_sum_subarray([100, 200, 300, 400].to_vec(), 2);

    println!("The this is happening: {:?}", result)
}


