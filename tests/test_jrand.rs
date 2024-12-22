use seedcracker::random::jrand::JRand;

#[test]
fn test_next_int_bound() {
    let mut rand = JRand::new(4506419895);
    assert_eq!(rand.get_next_int_bound(100), 10)
}

#[test]
fn test_next_bool() {
    let mut rand = JRand::new(4506419895);
    assert!(rand.get_next_bool());
}

#[test]
fn test_next_seed() {
    let mut rand = JRand::new(4506419895);
    assert_eq!(rand.next_seed(), 281474976500957)
}

#[test]
fn test_next_float() {
    let mut rand = JRand::new(4506419895);
    assert!((rand.get_next_float() - 0.99999994).abs() < f32::EPSILON)
}

#[test]
fn test_next_double() {
    let mut rand = JRand::new(4506419895);
    assert!((rand.get_next_double() - 0.9999999883008573).abs() < f64::EPSILON);
}

#[test]
fn test_shuffle() {
    let mut rand = JRand::new(4506419895);
    let mut vec = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ];
    rand.shuffle(&mut vec);
    assert_eq!(
        vec,
        [7, 11, 5, 15, 10, 17, 8, 18, 4, 14, 12, 2, 1, 3, 13, 6, 16, 9]
    )
}

#[test]
fn test_next_long() {
    let mut rand = JRand::new(4506419895);
    assert_eq!(rand.get_next_long(), -16256950163)
}