mod leetcode;
fn main() {
    // println!("Hello, world!");
    // let word = " ".to_string();

    // let result = leetcode::is_palindrome(word.clone());

    // let nums = vec![1, 2, 2];
    // let k = 3;
    let result = leetcode::longest_ones([1,1,1,0,0,0,1,1,1,1,0].to_vec(), 2);

    println!("The this is happening: {:?}", result)
}


