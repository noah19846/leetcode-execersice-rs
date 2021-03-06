pub struct NextGreaterElements;

impl NextGreaterElements {
  fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len();
    let mut result: Vec<i32> = Vec::new();

    for i in 0..length {
      let mut greater_num = -1;

      if i > 0 && i < length -1 && nums[i] == nums[i - 1] {
        greater_num = result[i - 1]
      } else {
        for j in i + 1..length + i {
          let real_index = if j > length - 1 {
            j - length
          } else {
            j
          };
          if nums[real_index] > nums[i] {
            greater_num = nums[real_index];
            break;
          }
        }
      }

      result.push(greater_num);
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let mut nums: Vec<i32> = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(1);
    nums.push(1);
    nums.push(3);
    nums.push(3);
    nums.push(3);
    let mut result: Vec<i32> = Vec::new();
    result.push(2);
    result.push(3);
    result.push(3);
    result.push(3);
    result.push(-1);
    result.push(-1);
    result.push(-1);
    assert_eq!(NextGreaterElements::next_greater_elements(nums), result);
  }
}
