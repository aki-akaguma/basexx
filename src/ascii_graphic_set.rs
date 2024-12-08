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
    binmap: Box<[u8]>,
    // ascii to binary map
    a128map: Box<[u8]>,
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
        assert!(a.len() <= 64);
        assert_eq!(a.iter().position(|&x| !x.is_ascii_graphic()), None);
        //let binmap = a.to_vec();
        let mut binmap = AlignedData64::alloc(64);
        binmap[0..a.len()].copy_from_slice(a);
        //let mut a128map: Vec<i8> = vec![-1; 128];
        let mut a128map = AlignedData128::alloc(128);
        a128map.fill(0xFF);
        for (idx, &a) in binmap.iter().enumerate() {
            a128map[a as usize] = idx as u8;
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
            if idx != 0xFF {
                return Some(idx);
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
        /*
        match self.position(ascii) {
            Some(binary) => Ok(binary),
            None => Err(DecodeError::InvalidByte(ascii)),
        }
        */
        if let Some(&binary) = self.a128map.get(ascii as usize) {
            if binary != 0xFF {
                return Ok(binary);
            }
        }
        Err(DecodeError::InvalidByte(ascii))
    }
    #[inline]
    pub fn getq(&self, binary: u8) -> Result<u8, EncodeError> {
        /*
        match self.get(binary) {
            Some(ascii) => Ok(ascii),
            None => Err(EncodeError::InvalidIndex(binary)),
        }
        */
        if let Some(&ascii) = self.binmap.get(binary as usize) {
            return Ok(ascii);
        }
        Err(EncodeError::InvalidIndex(binary))
    }
    #[allow(dead_code)]
    #[inline]
    pub fn binary_to_ascii(&self, buf: &mut [u8]) -> Result<(), EncodeError> {
        let buf = if buf.len() < 8 {
            buf
        } else {
            let mut iter = buf.chunks_exact_mut(8);
            let mut nx = iter.next();
            while let Some(bb) = nx {
                bb[0] = self.getq(bb[0])?;
                bb[1] = self.getq(bb[1])?;
                bb[2] = self.getq(bb[2])?;
                bb[3] = self.getq(bb[3])?;
                bb[4] = self.getq(bb[4])?;
                bb[5] = self.getq(bb[5])?;
                bb[6] = self.getq(bb[6])?;
                bb[7] = self.getq(bb[7])?;
                nx = iter.next();
            }
            iter.into_remainder()
        };
        for c in buf {
            *c = self.getq(*c)?;
        }
        Ok(())
    }
    #[allow(dead_code)]
    #[inline]
    pub fn ascii_to_binary(&self, buf: &mut [u8]) -> Result<(), DecodeError> {
        let buf = if buf.len() < 8 {
            buf
        } else {
            let mut iter = buf.chunks_exact_mut(8);
            let mut nx = iter.next();
            while let Some(bb) = nx {
                bb[0] = self.posq(bb[0])?;
                bb[1] = self.posq(bb[1])?;
                bb[2] = self.posq(bb[2])?;
                bb[3] = self.posq(bb[3])?;
                bb[4] = self.posq(bb[4])?;
                bb[5] = self.posq(bb[5])?;
                bb[6] = self.posq(bb[6])?;
                bb[7] = self.posq(bb[7])?;
                nx = iter.next();
            }
            iter.into_remainder()
        };
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
