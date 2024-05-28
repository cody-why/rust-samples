/*
 * @Date: 2024-05-17 15:18:38
 * @LastEditTime: 2024-05-17 15:23:12
 */

#![allow(unused_imports)]


#[test]
fn test_router() -> Result<(), Box<dyn std::error::Error>> {
    use matchit::Router;

    let mut router = Router::new();
    router.insert("/home", "Welcome!")?;
    router.insert("/users/{id}", "A User")?;
    router.insert("1001", "Number 1001!")?;

    let matched = router.at("/users/978")?;
    assert_eq!(matched.params.get("id"), Some("978"));
    assert_eq!(*matched.value, "A User");

    let matched = router.at("1001")?;
    assert_eq!(*matched.value, "Number 1001!");
    Ok(())
}