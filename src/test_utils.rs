pub const _CMAP64: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

pub const _CMAP32: [u8; 32] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'2', b'3', b'4', b'5', b'6', b'7',
];

pub const _CMAP58: [u8; 58] = [
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'J', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y',
    b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'm', b'n', b'o', b'p',
    b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];

#[allow(dead_code)]
pub fn read_t4_data() -> Vec<u8> {
    read_file_data("fixtures/t4.data")
}

#[allow(dead_code)]
pub fn read_t4_base64_ascii() -> String {
    read_file_ascii("fixtures/t4.base64.ascii")
}

#[allow(dead_code)]
pub fn read_t4_base64_pad_ascii() -> String {
    read_file_ascii("fixtures/t4.base64.pad.ascii")
}

#[allow(dead_code)]
pub fn read_t4_base32_ascii() -> String {
    read_file_ascii("fixtures/t4.base32.ascii")
}

#[allow(dead_code)]
pub fn read_file_data(fnm: &str) -> Vec<u8> {
    use std::io::Read;
    let mut v = Vec::new();
    let mut f = std::fs::File::open(fnm).unwrap();
    let _ = f.read_to_end(&mut v);
    v
}

#[allow(dead_code)]
pub fn read_file_ascii(fnm: &str) -> String {
    use std::io::Read;
    let mut v = Vec::new();
    let mut f = std::fs::File::open(fnm).unwrap();
    let _ = f.read_to_end(&mut v);
    let vv = if v[v.len() - 1] == b'\n' {
        &v[..(v.len() - 1)]
    } else {
        &v
    };
    String::from_utf8_lossy(vv).to_string()
}