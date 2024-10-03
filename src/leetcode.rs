use core::num;
use std::collections::{HashMap, HashSet};

pub fn is_palindrome(s: String) -> bool {
    let new_s = s.to_lowercase();
    let new_stg: Vec<char> = new_s.chars().filter(|c| c.is_alphanumeric()).collect();

    if new_stg.len() == 0 {
        return true;
    }
    let mut start_ptr = 0;
    let mut end_ptr = new_stg.len() - 1;


    while start_ptr < end_ptr {
        if new_stg[start_ptr] != new_stg[end_ptr] {
            return false;
        }
        start_ptr += 1;
        end_ptr -= 1;
    }
    true
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_arr = nums.clone();
    sorted_arr.sort();

    let mut result_set: Vec<Vec<i32>> = vec![];  // basically deals with the duplicates
    for (index, element) in sorted_arr.iter().enumerate() {
        
        if index > 0 && sorted_arr[index] == sorted_arr[index - 1] {
            continue;
        }

        let target = 0;
        let mut left = index + 1;
        let mut right = sorted_arr.len() - 1;

        while left < right {
            let current_sum = element + sorted_arr[left] + sorted_arr[right];
            if current_sum == target {
                let sum_arr = vec![element.to_owned(), sorted_arr[left], sorted_arr[right]];
                result_set.push(sum_arr);
                left += 1;
                
                while sorted_arr[left] == sorted_arr[left - 1] && left < right {
                    left += 1;
                }
            } else if current_sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    result_set
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut min_size_counter = nums.len() as i32 + 1;
    let mut sum_counter = 0;
    let mut left_ptr = 0 as usize;

    for right_ptr in 0..nums.len() {
        sum_counter += nums[right_ptr];

        while sum_counter >= target {
            min_size_counter = min_size_counter.min((right_ptr - left_ptr + 1) as i32);

            sum_counter -= nums[left_ptr];
            left_ptr += 1;
        }
    }

    if min_size_counter == nums.len() as i32 + 1 {
        0
    } else {
        min_size_counter
    }
}


pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hashset_max_len = 0;
    let mut hashset_current_count: usize;

    if s.len() == 1 {
        return 1
    }

    let mut left_ptr = 0 as usize;
    let mut hash_set = HashSet::new();

    for right_ptr in  0..s.len() {        
        while hash_set.contains(&s.chars().nth(right_ptr).unwrap()) {
            hash_set.remove(&s.chars().nth(left_ptr).unwrap());
            left_ptr += 1;
        }
        hash_set.insert(s.chars().nth(right_ptr).unwrap());
        hashset_current_count = right_ptr - left_ptr + 1;
        hashset_max_len = hashset_max_len.max(hashset_current_count);
    }

    hashset_max_len.try_into().unwrap()
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut curren_area: i32;

    let mut left_ptr = 0 as usize;
    let mut right_ptr = (height.len() - 1) as usize;

    while left_ptr < right_ptr {
        
        let height_con = height[right_ptr].min(height[left_ptr]);
        let  lenght = (right_ptr - left_ptr) as i32;

        curren_area = height_con * lenght;
        max_area = max_area.max(curren_area);

        if  height[left_ptr] < height[right_ptr] {
            left_ptr += 1
        } else {
            right_ptr -= 1
        }
    }
    max_area
}

//Just not to forget i could also write terrible code but i can learn to

// pub fn character_replacement(mut s: String, k: i32) -> i32 {
//     let mut max_substr_count = 0;
//     let mut current_count:i32;

//     let mut num_k: i32 = k;
//     let mut left_ptr = 0;
//     for (u, char) in s.clone().chars().enumerate() {
//         let left_char = s.chars().nth(left_ptr).unwrap();
//         if left_ptr <= u && left_char == char {
//             current_count = (u - left_ptr + 1) as i32;
//             max_substr_count = max_substr_count.max(current_count);
//             continue;
//         }
//         if num_k == 0 && left_char != char && left_ptr <= u {
//             left_ptr += 1;
//             continue;
//         } else {
//             num_k -= 1;
//             s = s.replacen(&char.to_string(),&left_char.to_string(), 1);
//             current_count = (u - left_ptr + 1) as i32;
//             max_substr_count = max_substr_count.max(current_count);
//             continue;
//         }
//         println!("These are the versions of s when left{left_char} the  {s}") 
//     }
//     max_substr_count
// }


pub fn character_replacement (s: String, k: i32) -> i32 {
    let mut char_count = HashMap::new();

    let mut max_freq_char = 0; 
    let mut left = 0;

    let mut max_len = 0;
    let chars:Vec<char> = s.chars().collect();

    for right in 0..s.len() {
        let right_char = chars[right];

        *char_count.entry(right_char).or_insert(0) +=1;
        max_freq_char = max_freq_char.max(*char_count.get(&right_char).unwrap());

        while (right - left + 1) as i32 - max_freq_char > k {
            let left_char = chars[left];
            *char_count.get_mut(&left_char).unwrap() -=1;
            left += 1;
        }
        max_len = max_len.max((right - left + 1) as i32);
    }
    max_len
}   

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_vec = vec![];
    let mut left = 0 as usize;
    let mut right = numbers.len() - 1 as usize;

    while left < right  {
        let current_sum = numbers[left] + numbers[right];
        if current_sum == target {
            sum_vec.extend(vec![left as i32  + 1, right as i32 + 1]);
            break;
        }
        if current_sum > target {
            right -= 1
        } else if current_sum < target {
            left += 1
        }
    }
    sum_vec
}

pub fn max_sum_subarray(numbers: Vec<i32>, k: i32) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;

    let mut left = 0 as usize;

    for right in 0..numbers.len() {
        current_sum +=  numbers[right];

        if (right - left + 1) as i32 > k {
            current_sum -= numbers[left];
            left += 1;
        }
        if (right - left + 1) as i32 == k {
            max_sum = max_sum.max(current_sum);
        }
    }
    max_sum
}