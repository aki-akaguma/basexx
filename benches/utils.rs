use std::io::Read;

pub fn read_data_t1() -> Vec<u8> {
    let mut v = Vec::new();
    let mut f = std::fs::File::open("fixtures/t4.data").unwrap();
    let _ = f.read_to_end(&mut v);
    v
}
