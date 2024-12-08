//
const AD64SZ: usize = 64;

#[derive(Debug)]
#[repr(align(64))]
struct AlignedData64 {
    #[allow(unused)]
    data: [u8; AD64SZ],
}

impl Default for AlignedData64 {
    fn default() -> Self {
        AlignedData64 { data: [0; AD64SZ] }
    }
}

impl AlignedData64 {
    #[allow(dead_code)]
    pub fn alloc(size: usize) -> Box<[u8]> {
        let size = size.div_ceil(AD64SZ);
        let mut vec = Vec::<AlignedData64>::with_capacity(size);
        vec.resize_with(size, Default::default);
        let data = unsafe {
            #[allow(clippy::unsound_collection_transmute)]
            let mut data = std::mem::transmute::<Vec<AlignedData64>, Vec<u8>>(vec);
            data.set_len(size * AD64SZ);
            data
        };
        data.into_boxed_slice()
    }
}

//
const AD128SZ: usize = 128;

#[derive(Debug)]
#[repr(align(128))]
struct AlignedData128 {
    #[allow(unused)]
    data: [u8; AD128SZ],
}

impl Default for AlignedData128 {
    fn default() -> Self {
        AlignedData128 { data: [0; AD128SZ] }
    }
}

impl AlignedData128 {
    #[allow(dead_code)]
    pub fn alloc(size: usize) -> Box<[u8]> {
        let size = size.div_ceil(AD128SZ);
        let mut vec = Vec::<AlignedData128>::with_capacity(size);
        vec.resize_with(size, Default::default);
        let data = unsafe {
            #[allow(clippy::unsound_collection_transmute)]
            let mut data = std::mem::transmute::<Vec<AlignedData128>, Vec<u8>>(vec);
            data.set_len(size * AD128SZ);
            data
        };
        data.into_boxed_slice()
    }
}
#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    //
    #[test]
    fn it_works_0() {
        let mem0 = AlignedData64::alloc(100);
        let addr = mem0.as_ptr() as *const u8 as u64;
        assert_eq!(addr % 64, 0);
        assert!(mem0.len() >= 100);
    }
    #[test]
    fn it_works_1() {
        let mem0 = AlignedData64::alloc(1000);
        let addr = mem0.as_ptr() as *const u8 as u64;
        assert_eq!(addr % 64, 0);
        assert!(mem0.len() >= 1000);
    }
    #[test]
    fn it_works_2() {
        let mem0 = AlignedData64::alloc(1);
        let addr = mem0.as_ptr() as *const u8 as u64;
        assert_eq!(addr % 64, 0);
        assert_eq!(mem0.len(), 64);
    }
    #[test]
    fn it_works_3() {
        let mem0 = AlignedData128::alloc(100);
        let addr = mem0.as_ptr() as *const u8 as u64;
        assert_eq!(addr % 128, 0);
        assert!(mem0.len() >= 100);
    }
    #[test]
    fn it_works_4() {
        let mem0 = AlignedData128::alloc(1000);
        let addr = mem0.as_ptr() as *const u8 as u64;
        assert_eq!(addr % 128, 0);
        assert!(mem0.len() >= 1000);
    }
    #[test]
    fn it_works_5() {
        let mem0 = AlignedData128::alloc(1);
        let addr = mem0.as_ptr() as *const u8 as u64;
        assert_eq!(addr % 128, 0);
        assert_eq!(mem0.len(), 128);
    }
}
