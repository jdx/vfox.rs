#[test]
fn test_available() {
    let result = vfox::hooks::available();
    assert_eq!(result, Vec::from(["one".to_string(), "two".to_string()]));
}
