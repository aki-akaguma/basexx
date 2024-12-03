use std::io::Read;

#[allow(dead_code)]
pub fn read_data_t1() -> Vec<u8> {
    let mut v = Vec::new();
    let mut f = std::fs::File::open("fixtures/t4.data").unwrap();
    let _ = f.read_to_end(&mut v);
    v
}

#[allow(dead_code)]
pub fn read_ascii_t1() -> Vec<u8> {
    let mut v = Vec::new();
    let mut f = std::fs::File::open("fixtures/t4.base64.ascii").unwrap();
    let _ = f.read_to_end(&mut v);
    if v[v.len() - 1] == b'\n' {
        v.pop();
    };
    v
}
