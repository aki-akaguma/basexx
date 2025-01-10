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

mod ags_scalar;
pub(crate) use ags_scalar::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod ags_128_ssse3;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use ags_128_ssse3::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod ags_128_avx2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use ags_128_avx2::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod ags_64_ssse3;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use ags_64_ssse3::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod ags_64_avx2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use ags_64_avx2::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod ags_32_ssse3;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use ags_32_ssse3::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod ags_32_avx2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use ags_32_avx2::*;

#[cfg(feature = "aligned_data")]
#[derive(Debug)]
pub(crate) struct AsciiGraphicSet {
    // binary to ascii map
    binmap: Box<[u8]>,
    // ascii to binary map
    a128map: Box<[u8]>,
}

#[cfg(not(feature = "aligned_data"))]
#[derive(Debug)]
pub(crate) struct AsciiGraphicSet {
    // binary to ascii map
    binmap: Vec<u8>,
    // ascii to binary map
    a128map: Vec<u8>,
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
    #[cfg(not(feature = "aligned_data"))]
    #[allow(dead_code)]
    pub fn with_slice(a: &[u8]) -> Self {
        assert!(a.len() <= 64);
        assert_eq!(a.iter().position(|&x| !x.is_ascii_graphic()), None);
        let binmap = a.to_vec();
        let mut a128map: Vec<u8> = vec![0xFF; 128];
        for (idx, &a) in binmap.iter().enumerate() {
            a128map[a as usize] = idx as u8;
        }
        Self { binmap, a128map }
    }
    #[cfg(feature = "aligned_data")]
    #[allow(dead_code)]
    pub fn with_slice(a: &[u8]) -> Self {
        assert!(a.len() <= 64);
        assert_eq!(a.iter().position(|&x| !x.is_ascii_graphic()), None);
        let mut binmap = AlignedData64::alloc(64);
        binmap[0..a.len()].copy_from_slice(a);
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
    #[inline(always)]
    pub fn position(&self, byte: u8) -> Option<u8> {
        //self.cmap.iter().position(|&x| x == byte).map(|idx| idx as u8)
        if let Some(&idx) = self.a128map.get(byte as usize) {
            if idx != 0xFF {
                return Some(idx);
            }
        }
        None
    }
    #[inline(always)]
    pub fn get(&self, index: u8) -> Option<u8> {
        self.binmap.get(index as usize).copied()
    }
}

impl AsciiGraphicSet {
    #[inline(always)]
    pub fn posq(&self, ascii: u8) -> Result<u8, DecodeError> {
        if let Some(&binary) = self.a128map.get(ascii as usize) {
            if binary != 0xFF {
                return Ok(binary);
            }
        }
        Err(DecodeError::InvalidByte(ascii))
    }
    #[inline(always)]
    pub fn getq(&self, binary: u8) -> Result<u8, EncodeError> {
        if let Some(&ascii) = self.binmap.get(binary as usize) {
            return Ok(ascii);
        }
        Err(EncodeError::InvalidIndex(binary))
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn binary_to_ascii(&self, buf: &mut [u8]) -> Result<(), EncodeError> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if cfg!(target_feature = "sse2") {
            if is_x86_feature_detected!("avx2") {
                if self.len() == 64 {
                    unsafe { _binary_to_ascii_64_avx2(&self.binmap, buf) }
                } else if self.len() == 32 {
                    unsafe { _binary_to_ascii_32_avx2(&self.binmap, buf) }
                } else {
                    _binary_to_ascii_scalar(&self.binmap, buf)
                }
            } else if is_x86_feature_detected!("ssse3") {
                if self.len() == 64 {
                    unsafe { _binary_to_ascii_64_ssse3(&self.binmap, buf) }
                } else if self.len() == 32 {
                    unsafe { _binary_to_ascii_32_ssse3(&self.binmap, buf) }
                } else {
                    _binary_to_ascii_scalar(&self.binmap, buf)
                }
            } else {
                _binary_to_ascii_scalar(&self.binmap, buf)
            }
        } else {
            _binary_to_ascii_scalar(&self.binmap, buf)
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            _binary_to_ascii_scalar(&self.binmap, buf)
        }
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn ascii_to_binary(&self, buf: &mut [u8]) -> Result<(), DecodeError> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if cfg!(target_feature = "sse2") {
            if is_x86_feature_detected!("avx2") {
                unsafe { _ascii_to_binary_128_avx2(&self.a128map, buf) }
            } else if is_x86_feature_detected!("ssse3") {
                unsafe { _ascii_to_binary_128_ssse3(&self.a128map, buf) }
            } else {
                _ascii_to_binary_scalar(&self.a128map, buf)
            }
        } else {
            _ascii_to_binary_scalar(&self.a128map, buf)
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            _ascii_to_binary_scalar(&self.a128map, buf)
        }
    }
    //
    #[cfg(target_feature = "sse2")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn binary_to_ascii_64_ssse3(&self, buf: &mut [u64; 2]) -> Result<(), EncodeError> {
        assert!(self.len() == 64);
        unsafe { _binary_to_ascii_64_ssse3_c16(&self.binmap, buf) }
    }
    #[cfg(target_feature = "sse2")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn ascii_to_binary_64_ssse3(&self, buf: &mut [u64; 2]) -> Result<(), DecodeError> {
        assert!(self.len() == 64);
        unsafe { _ascii_to_binary_128_ssse3_c16(&self.a128map, buf) }
    }
    //
    #[cfg(target_feature = "sse2")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn binary_to_ascii_64_avx2(&self, buf: &mut [u64; 4]) -> Result<(), EncodeError> {
        assert!(self.len() == 64);
        unsafe { _binary_to_ascii_64_avx2_c32(&self.binmap, buf) }
    }
    #[cfg(target_feature = "sse2")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn ascii_to_binary_64_avx2(&self, buf: &mut [u64; 4]) -> Result<(), DecodeError> {
        assert!(self.len() == 64);
        unsafe { _ascii_to_binary_128_avx2_c32(&self.a128map, buf) }
    }
    //
    #[cfg(target_feature = "sse2")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn binary_to_ascii_32_ssse3(&self, buf: &mut [u64; 2]) -> Result<(), EncodeError> {
        assert!(self.len() == 32);
        unsafe { _binary_to_ascii_32_ssse3_c16(&self.binmap, buf) }
    }
    #[cfg(target_feature = "sse2")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn ascii_to_binary_32_ssse3(&self, buf: &mut [u64; 2]) -> Result<(), DecodeError> {
        assert!(self.len() == 32);
        unsafe { _ascii_to_binary_128_ssse3_c16(&self.a128map, buf) }
    }
    //
    #[cfg(target_feature = "sse2")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn binary_to_ascii_32_avx2(&self, buf: &mut [u64; 4]) -> Result<(), EncodeError> {
        assert!(self.len() == 32);
        unsafe { _binary_to_ascii_32_avx2_c32(&self.binmap, buf) }
    }
    #[cfg(target_feature = "sse2")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn ascii_to_binary_32_avx2(&self, buf: &mut [u64; 4]) -> Result<(), DecodeError> {
        assert!(self.len() == 32);
        unsafe { _ascii_to_binary_128_avx2_c32(&self.a128map, buf) }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_ascii_graphic_set_1() {
        let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
        //
        assert_eq!(ags.len(), 64);
        //
        assert_eq!(ags.get(0), Some(b'A'));
        assert_eq!(ags.get(1), Some(b'B'));
        assert_eq!(ags.get(26), Some(b'a'));
        assert_eq!(ags.get(27), Some(b'b'));
        assert_eq!(ags.get(52), Some(b'0'));
        assert_eq!(ags.get(53), Some(b'1'));
        assert_eq!(ags.get(62), Some(b'+'));
        assert_eq!(ags.get(63), Some(b'/'));
        //
        assert_eq!(ags.position(b'A'), Some(0));
        assert_eq!(ags.position(b'B'), Some(1));
        assert_eq!(ags.position(b'a'), Some(26));
        assert_eq!(ags.position(b'b'), Some(27));
        assert_eq!(ags.position(b'0'), Some(52));
        assert_eq!(ags.position(b'1'), Some(53));
        assert_eq!(ags.position(b'+'), Some(62));
        assert_eq!(ags.position(b'/'), Some(63));
    }

    #[test]
    fn test_ascii_graphic_set_binary_to_ascii() {
        let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
        let mut buf = Vec::<u8>::with_capacity(64);
        for i in 0..64 {
            buf.push(i);
        }
        let r = ags.binary_to_ascii(&mut buf);
        assert!(r.is_ok());
        assert_eq!(buf, &test_utils::_CMAP64);
    }

    #[test]
    fn test_ascii_graphic_set_ascii_to_binary() {
        let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
        let mut valid = Vec::<u8>::with_capacity(64);
        for i in 0..64 {
            valid.push(i);
        }
        let mut buf = Vec::<u8>::with_capacity(64);
        for i in 0..64 {
            buf.push(test_utils::_CMAP64[i]);
        }
        let r = ags.ascii_to_binary(&mut buf);
        assert!(r.is_ok());
        assert_eq!(buf, valid);
    }
}
