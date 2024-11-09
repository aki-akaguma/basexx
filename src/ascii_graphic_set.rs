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

/*
impl Default for AsciiGraphicSet {
    fn default() -> Self {
        AsciiGraphicSet::new()
    }
}
*/

impl AsciiGraphicSet {
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
