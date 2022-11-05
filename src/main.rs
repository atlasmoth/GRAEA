
fn main() {
  
  extended_euclidean(51,11);
     
}

#[allow(dead_code)]
 #[derive(Debug)]
 pub struct AlgoResult{
  gcd : i32,
  multiplicative_inverse : i32,
  y_coefficient : i32
 }
/***
 *@Description - The function calculates the modular multiplicative inverse based on Bezout's coefficients, derived from :
 *    ax  + by = gcd(a,b)
 * For a clearer description, visit http://www-math.ucdenver.edu/~wcherowi/courses/m5410/exeucalg.* html

 * last_second_auxilliary_variable  is returned as the 
 */
pub fn extended_euclidean(first_num:i32,last_num:i32) -> AlgoResult{

  // initialize all base variables
  let mut current_remainder = last_num;
  let mut last_remainder = first_num;
  let mut current_first_auxilliary_variable = 0;

  let mut last_first_auxilliary_variable = 1; 
  // the first value for the first auxilliary variable is 0 while its next value is 1
  let mut current_second_auxilliary_variable = 1;
  let mut last_second_auxilliary_variable = 0;
  // the first value for the second auxilliary variable is 1 while its next value is 0
  /*
 * last_remainder is ultimately returned as the GCD
 * last_first_auxilliary_variable is ultimately returned as the modular multiplicative inverse

  */
  // Once current_remainder is zero, terminate the loop, return AlgoResult struct with appropriate values
  while current_remainder != 0 {
/*
*  Basically the logic here is :
*  Variables beginning with "current" are updated by subtracting  integer(last_remainder /    current_remainder) multiplied by their current value from the last value of those variables. 
 Then the current value is set as the last value.
*/


    // calculate the integer ratio between the previous remainder and current remainder
    let ratio = last_remainder / current_remainder;
    let temp_remainder = current_remainder;

    current_remainder = last_remainder - ratio * current_remainder;
    last_remainder = temp_remainder;
    // iteratively updating auxilliary variables - first aux variable
    let temp_first_auxilliary_variable = current_first_auxilliary_variable;
    current_first_auxilliary_variable = last_first_auxilliary_variable - ratio * current_first_auxilliary_variable;
    last_first_auxilliary_variable = temp_first_auxilliary_variable;
    // iteratively updating auxilliary variables - second aux variable
    let temp_second_auxilliary_variable = current_second_auxilliary_variable;
    current_second_auxilliary_variable = last_second_auxilliary_variable - ratio * current_second_auxilliary_variable;
    last_second_auxilliary_variable = temp_second_auxilliary_variable;
  }
  println!("gcd = {},modular multiplicative inverse or first coefficient of Bezout's equation = {},second coefficient of Bezout's equation = {}",last_remainder,last_first_auxilliary_variable,last_second_auxilliary_variable);
  AlgoResult{
    gcd:last_remainder,
    multiplicative_inverse:last_first_auxilliary_variable,
    y_coefficient:last_second_auxilliary_variable
  }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extended_euclidean() {
        let result = extended_euclidean(35,8);
        assert_eq!(result.gcd,1);
        assert_eq!(result.multiplicative_inverse,3);
        assert_eq!(result.y_coefficient,-13);
    }

    
}