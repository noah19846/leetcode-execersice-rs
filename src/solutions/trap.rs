pub struct Solution;

impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let len = height.len();

    if len < 3 {
      return 0;
    }

    let mut result: i32 = 0;
    let mut left_highest: i32 = height[0];
    let mut right_highest: i32 = 0;
    let mut right_highest_index = 0;

    for i in 1..len {
      if height[i] > right_highest {
        right_highest = height[i];
        right_highest_index = i;
      }
    }

    for i in 1..len - 1 {
      left_highest = if height[i - 1] < left_highest {
        left_highest
      } else {
        height[i - 1]
      };

      if i == right_highest_index {
        if right_highest > left_highest {
          left_highest = right_highest;
        }
        right_highest = 0;

        for j in i + 1..len {
          if height[j] > right_highest {
            right_highest = height[j];
            right_highest_index = j;
          }
        }
      } else {
        let lower = if left_highest > right_highest {
          right_highest
        } else {
          left_highest
        };

        if height[i] < lower {
          let delta = lower - height[i];
          result += delta;
        }
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn t() {
    let mut height: Vec<i32> = Vec::new();
    let mut height1: Vec<i32> = Vec::new();

    height.push(2);
    height.push(1);
    height.push(0);
    height.push(1);
    height.push(3);
    height.push(2);
    height.push(1);
    height.push(2);
    height.push(1);
    height.push(2);

    height1.push(10000);

    assert_eq!(Solution::trap(height), 6);
    assert_eq!(Solution::trap(height1), 0);
  }
}
