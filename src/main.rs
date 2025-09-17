mod array;

use array::array_1::Solution;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    // 通过完整的路径调用 two_sum 方法
    let result = Solution::two_sum(nums, target);

    println!("Indices are: {:?}", result);
}
