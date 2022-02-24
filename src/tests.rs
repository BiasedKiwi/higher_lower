use super::*;

#[test]
fn test_to_int() {
    let s = String::from("123");
    assert_eq!(to_int(&s), 123);
}

#[test]
#[should_panic]
fn test_to_int_fail() {
    let s = String::from("foo");
    assert_eq!(to_int(&s), 0);
}

#[test]
fn test_to_int_negative() {
    let s = String::from("-123");
    assert_eq!(to_int(&s), -123);
}

#[test]
fn test_is_numeric() {
    let s = String::from("123");
    assert!(check_if_numeric(&s));
}

#[test]
fn test_is_numeric_negative() {
    let s = String::from("-123");
    assert!(check_if_numeric(&s));
}



#[test]
#[should_panic]
fn test_is_numeric_alpha_and_numeric_fail() {
    let s = String::from("123a");
    check_if_numeric(&s);
}

#[test]
#[should_panic]
fn test_is_numeric_alpha_only_fail() {
    let s = String::from("foo");
    check_if_numeric(&s);
}
