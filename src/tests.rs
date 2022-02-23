use super::*;

#[test]
fn test_to_int() {
    let s = String::from("123");
    assert_eq!(to_int(&s), 123);
}

#[test]
fn test_is_numeric() {
    let s = String::from("123");
    assert_eq!(check_if_numeric(&s), true);
}

#[test]
#[should_panic]
fn test_is_numeric_fail() {
    let s = String::from("123a");
    check_if_numeric(&s);
}