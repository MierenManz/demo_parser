use demo_parser::reader::Reader;
use demo_parser::structs::demo::Demo;

#[test]
fn new() {
  let mut reader = Reader::new_from_path("./tests/test_data/test2.dem").unwrap();
  let demo = Demo::new(&mut reader).unwrap();
}
