use seedcracker::check_seed;

#[test]
fn test_check_seed() {
    assert!(check_seed((-4872636734044769429_i64) as u64, -28, -73));
}