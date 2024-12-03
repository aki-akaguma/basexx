pub fn read_file_data(fnm: &str) -> Vec<u8> {
    use std::io::Read;
    let mut v = Vec::new();
    let mut f = std::fs::File::open(fnm).unwrap();
    let _ = f.read_to_end(&mut v);
    v
}

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

