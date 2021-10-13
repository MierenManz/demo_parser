use demo_parser::reader::Reader;

#[test]
fn new() {
    Reader::new(vec![]);
}

#[test]
fn new_from_path() {
    Reader::new_from_path("./tests/test_data/test.txt").unwrap();
}
#[test]
fn read() {
    let mut r = Reader::new_from_path("./tests/test_data/test.txt").unwrap();
    let buff = r.read(9);
    assert_eq!(String::from_utf8(buff).unwrap(), "some data");
}
