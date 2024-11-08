/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Graphic_character
 *     https://en.wikipedia.org/wiki/ASCII
*/

/*
#[derive(Debug)]
pub enum AgsError {
    NotFound(u8),
}
*/

#[derive(Debug)]
pub struct AsciiGraphicSet {
    cmap: Vec<u8>,
}

impl Default for AsciiGraphicSet {
    fn default() -> Self {
        AsciiGraphicSet::new()
    }
}

impl AsciiGraphicSet {
    pub fn new() -> Self {
        Self::with_slice(&_CMAP64)
    }
    #[allow(dead_code)]
    pub fn with_str(a: &str) -> Self {
        Self::with_slice(a.as_bytes())
    }
    pub fn with_slice(a: &[u8]) -> Self {
        assert!(a.len() <= u8::MAX as usize);
        assert_eq!(a.iter().position(|&x| !x.is_ascii_graphic()), None);
        Self { cmap: a.to_vec() }
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cmap.len()
    }
    pub fn position(&self, byte: u8) -> Option<u8> {
        self.cmap
            .iter()
            .position(|&x| x == byte)
            .map(|idx| idx as u8)
    }
    pub fn get(&self, index: u8) -> Option<u8> {
        self.cmap.get(index as usize).copied()
    }
}

const _CMAP64: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];
