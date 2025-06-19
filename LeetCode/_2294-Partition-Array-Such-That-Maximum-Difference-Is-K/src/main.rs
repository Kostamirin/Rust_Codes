pub struct Solution;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut answer = 0;
        nums.sort();
        
        let mut pointer = 0;
        for i in 0..nums.len() {
            let mut j = i;
            while nums[j] - nums[pointer] <= k && j <= nums.len() -1 {
                if j == nums.len() -1 {
                    return answer +1
                }
                j += 1;
            }
            pointer = j;
            answer += 1;
        }
        
        answer 
    }
    
}

fn main() {
    let nums = vec![2,2,4,5];
    let k = 0;
    let answer = Solution::partition_array(nums, k);
    println!("answer = {}", answer);
}
