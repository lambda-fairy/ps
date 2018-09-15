use std::collections::HashMap;

#[test]
fn identity() {
    assert_eq!(ps::identity().cast("hello"), "hello");
}

#[test]
fn integers() {
    assert_eq!(ps::i8_u8().cast(-1), 255);
}

#[test]
fn then() {
    assert_eq!(ps::i8_u8().then(ps::u8_i8()).cast(-1), -1);
}

#[test]
fn as_ref() {
    assert_eq!(ps::i8_u8().as_ref().cast(&-1), &255);
}

#[test]
fn as_mut() {
    assert_eq!(ps::i8_u8().as_mut().cast(&mut -1), &mut 255);
}

#[test]
fn hash_map_values() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("hello".into(), "world".into());
    let map: HashMap<String, Vec<u8>> =
        ps::hash_map_values(ps::string_bytes()).cast(map);
    assert_eq!(map["hello"], b"world");
}
