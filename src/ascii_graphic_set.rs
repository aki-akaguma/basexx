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
use super::*;

#[derive(Debug)]
pub(crate) struct AsciiGraphicSet {
    // binary to ascii map
    binmap: Vec<u8>,
    // ascii to binary map
    a128map: Vec<i8>,
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
    #[inline]
    pub fn with_str(a: &str) -> Self {
        Self::with_slice(a.as_bytes())
    }
    #[allow(dead_code)]
    pub fn with_slice(a: &[u8]) -> Self {
        assert!(a.len() <= u8::MAX as usize);
        assert_eq!(a.iter().position(|&x| !x.is_ascii_graphic()), None);
        let binmap = a.to_vec();
        let mut a128map: Vec<i8> = vec![-1; 128];
        for (idx, &a) in binmap.iter().enumerate() {
            a128map[a as usize] = idx as i8;
        }
        Self { binmap, a128map }
    }
    #[allow(dead_code)]
    #[inline]
    pub fn len(&self) -> usize {
        self.binmap.len()
    }
    #[inline]
    pub fn position(&self, byte: u8) -> Option<u8> {
        //self.cmap.iter().position(|&x| x == byte).map(|idx| idx as u8)
        if let Some(&idx) = self.a128map.get(byte as usize) {
            if idx >= 0 {
                return Some(idx as u8);
            }
        }
        None
    }
    #[inline]
    pub fn get(&self, index: u8) -> Option<u8> {
        self.binmap.get(index as usize).copied()
    }
    #[inline]
    pub fn posq(&self, ascii: u8) -> Result<u8, DecodeError> {
        match self.position(ascii) {
            Some(binary) => Ok(binary),
            None => Err(DecodeError::InvalidByte(ascii)),
        }
    }
    #[inline]
    pub fn getq(&self, binary: u8) -> Result<u8, EncodeError> {
        match self.get(binary) {
            Some(ascii) => Ok(ascii),
            None => Err(EncodeError::InvalidIndex(binary)),
        }
    }
    #[allow(dead_code)]
    pub fn binary_to_ascii(&self, buf: &mut [u8]) -> Result<(), EncodeError> {
        for c in buf {
            *c = self.getq(*c)?;
        }
        Ok(())
    }
    #[allow(dead_code)]
    pub fn ascii_to_binary(&self, buf: &mut [u8]) -> Result<(), DecodeError> {
        for c in buf {
            *c = self.posq(*c)?;
        }
        Ok(())
    }
}

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
