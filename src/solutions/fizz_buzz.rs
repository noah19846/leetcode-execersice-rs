pub struct FizzBuzz;

impl FizzBuzz {
  fn fizz_buzz(n: u32) -> Vec<String> {
    let mut i: u32 = 1;
    let mut result: Vec<String> = Vec::new();

    while i <= n {
      i += 1;
      result.push(FizzBuzz::get_fb_string(i));
    }

    result
  }

  fn get_fb_string(n: u32) -> String {
    let s;

    if n % 3 == 0 {
      if n % 5 != 0 {
        s = String::from("Fizz");
      } else {
        s = String::from("FizzBuzz");
      }
    } else if n % 5 == 0 {
      s = String::from("Buzz");
    } else {
      s = String::from(format!("{}", n));
    }

    s
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(FizzBuzz::get_fb_string(0), "FizzBuzz");
    assert_eq!(FizzBuzz::get_fb_string(3), "Fizz");
    assert_eq!(FizzBuzz::get_fb_string(5), "Buzz");
    assert_eq!(FizzBuzz::get_fb_string(11), "11");
    assert_eq!(FizzBuzz::get_fb_string(15), "FizzBuzz");
    assert_eq!(FizzBuzz::get_fb_string(15555), "FizzBuzz");
  }
}
