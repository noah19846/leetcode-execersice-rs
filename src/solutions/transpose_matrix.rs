pub struct MaxtrixTranspose;

impl MaxtrixTranspose {
  fn transpose(matrix: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let width = matrix.len();
    let height = matrix[0].len();
    let mut result: Vec<Vec<usize>> = Vec::new();

    for i in 0..height {
      result.push(Vec::new());

      for j in 0..width {
        result[i].push(matrix[j][i]);
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let mut m: Vec<Vec<usize>> = Vec::new();
    let mut result: Vec<Vec<usize>> = Vec::new();
    let width = 5;
    let height = 10;

    for i in 0..width {
      result.push(Vec::new());

      for j in 0..height {
        result[i].push(j);
      }
    }

    for i in 0..height {
      m.push(Vec::new());

      for _j in 0..width {
        m[i].push(i);
      }
    }

    assert_eq!(MaxtrixTranspose::transpose(m), result);
  }
}
