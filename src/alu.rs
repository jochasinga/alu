pub fn abs(num: i8) -> i8 {
    let mut n = num;
    match n.leading_zeros() {
	0 => !n + 1,
	_ => n,
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    
    #[test]
    pub fn test_abs() {
	// Just set to the last possible positive value
	let last_neg: i8 = -100;
	let last_pos: i8 = 120;

	assert_eq!(abs(last_neg), 100);
	assert_eq!(abs(last_pos), 120);
    }
}


