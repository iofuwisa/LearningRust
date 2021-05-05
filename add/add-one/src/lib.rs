//! Add one
//! Add one some number

/// add one
/// 
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}
