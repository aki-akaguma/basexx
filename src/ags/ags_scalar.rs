use super::super::*;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn _binary_to_ascii_scalar(lup: &[u8], buf: &mut [u8]) -> Result<(), EncodeError> {
    let buf = if buf.len() < 8 {
        buf
    } else {
        let mut iter = buf.chunks_exact_mut(8);
        let mut nx = iter.next();
        while let Some(bb) = nx {
            bb[0] = lup[bb[0] as usize];
            bb[1] = lup[bb[1] as usize];
            bb[2] = lup[bb[2] as usize];
            bb[3] = lup[bb[3] as usize];
            bb[4] = lup[bb[4] as usize];
            bb[5] = lup[bb[5] as usize];
            bb[6] = lup[bb[6] as usize];
            bb[7] = lup[bb[7] as usize];
            nx = iter.next();
        }
        iter.into_remainder()
    };
    /*
    let buf = if buf.len() < 4 {
        buf
    } else {
        let mut iter = buf.chunks_exact_mut(4);
        let mut nx = iter.next();
        while let Some(bb) = nx {
            bb[0] = lup[bb[0] as usize];
            bb[1] = lup[bb[1] as usize];
            bb[2] = lup[bb[2] as usize];
            bb[3] = lup[bb[3] as usize];
            nx = iter.next();
        }
        iter.into_remainder()
    };
    */
    for c in buf {
        *c = lup[*c as usize];
    }
    Ok(())
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn _ascii_to_binary_scalar(lup: &[u8], buf: &mut [u8]) -> Result<(), DecodeError> {
    macro_rules! step1 {
        ($target: expr) => {{
            let a = $target;
            let aa = lup[a as usize];
            $target = if aa != 0xFF {
                aa
            } else {
                return Err(DecodeError::InvalidByte(a));
            };
        }};
    }
    let buf = if buf.len() < 8 {
        buf
    } else {
        let mut iter = buf.chunks_exact_mut(8);
        let mut nx = iter.next();
        while let Some(bb) = nx {
            step1!(bb[0]);
            step1!(bb[1]);
            step1!(bb[2]);
            step1!(bb[3]);
            step1!(bb[4]);
            step1!(bb[5]);
            step1!(bb[6]);
            step1!(bb[7]);
            nx = iter.next();
        }
        iter.into_remainder()
    };
    /*
    let buf = if buf.len() < 4 {
        buf
    } else {
        let mut iter = buf.chunks_exact_mut(4);
        let mut nx = iter.next();
        while let Some(bb) = nx {
            step1!(bb[0]);
            step1!(bb[1]);
            step1!(bb[2]);
            step1!(bb[3]);
            nx = iter.next();
        }
        iter.into_remainder()
    };
    */
    for c in buf {
        step1!(*c);
    }
    Ok(())
}

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
