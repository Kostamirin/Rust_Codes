pub struct Solution;



impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = (nums[0] - nums[n - 1]).abs();

        for i in 0..n - 1 {
            result = result.max((nums[i] - nums[i + 1]).abs());
        }
        
        result
    }
}


fn main()
{
    println!("{:?}", Solution::max_adjacent_distance(vec![1,2,3,4,5]));
}
