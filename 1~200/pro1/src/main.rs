use std::collections::HashMap;
fn main() {
    // let nums : Vec<i32>= Vec::new();
    let nums = vec![1, 2, 3, 4, 5];
    // let new_nums = two_num(nums, 1);
    // for i in nums {
    //     println!("i : {}", i);
    // }
    for i in 0..nums.len() {
        println!("index: {}, value: {}", i, nums.get(i).unwrap());
    }
}

pub fn two_num(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut val_to_idx : HashMap<i32, i32> = HashMap::new();
    let len = nums.len();
    for i in 0..len {
        let tmp = val_to_idx.get(&(target - nums.get(i).unwrap()));
        if tmp.is_some() {
            return vec![*tmp.unwrap(),i as i32];
        } else {
            val_to_idx .insert( *nums.get(i).unwrap(), i as i32);
        }
    }
    vec![]
}
