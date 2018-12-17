extern crate nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(1), Ok(2));
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(2), Ok(3));
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::pi(1), 0);
    assert_eq!(np::pi(2), 1);
    assert_eq!(np::pi(3), 2);
    assert_eq!(np::pi(4), 4);
    assert_eq!(np::pi(5), 5);
    assert_eq!(np::pi(6), 5);
    assert_eq!(np::nth(3), Ok(5));
    // assert_eq!(np::nth(4), Ok(7));
    // assert_eq!(np::nth(5), Ok(11));
    // assert_eq!(np::nth(6), Ok(13));
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10001), Ok(104743));
}

#[test]
#[ignore]
fn test_zeroth_prime() {
    assert!(np::nth(0).is_err());
}
