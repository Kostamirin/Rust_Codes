pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let size = nums.len();
        for i in 0..size {
            if nums[i] == target {
                return i as i32
            }
            else if nums[i] < target {
                if i == size - 1 {
                    return size as i32
                }
                else{
                    continue;
                }
            }
            else {
                return i as i32
            }
        }
        size as i32 - 1
    }
}

fn main() {
    let nums = vec![1,3,5,6];
    let target = 7;
    let nums2 = vec![1];
    let target2 = 0;
    let nums3 = vec![1,3];
    let target3 = 2;
    
    println!("---HERE'RE THE RESULTS ---");
    println!("{:?}", Solution::search_insert(nums, target));
    println!("{:?}", Solution::search_insert(nums2, target2));
    println!("{:?}", Solution::search_insert(nums3, target3));
}
