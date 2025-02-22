#[cfg(test)]
mod test {
    use deku::prelude::*;
    use std::convert::TryFrom;
    use std::convert::TryInto;

    /// Basic test struct
    #[derive(Clone, Copy, PartialEq, Eq, Debug, DekuWrite, DekuRead)]
    #[deku(type = "u8")]
    #[non_exhaustive]
    #[repr(u8)]
    pub enum BasicMapping {
        /// A
        A = 0,
        /// B
        B = 1,
        /// C
        #[deku(default)]
        C = 2,
    }

    /// Advanced test struct
    #[derive(Clone, Copy, PartialEq, Eq, Debug, DekuWrite, DekuRead)]
    #[deku(type = "u8")]
    #[non_exhaustive]
    #[repr(u8)]
    pub enum AdvancedRemapping {
        /// A
        #[deku(id = "1")]
        A = 0,
        /// B
        #[deku(id = "2")]
        B = 1,
        /// C
        #[deku(id = "3", default)]
        C = 2,
    }

    #[test]
    fn test_basic_a() {
        let input = [0u8];
        let ret_read = BasicMapping::try_from(input.as_slice()).unwrap();
        assert_eq!(BasicMapping::A, ret_read);
        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(input.to_vec(), ret_write);
    }

    #[test]
    fn test_basic_c() {
        let input = [2u8];
        let ret_read = BasicMapping::try_from(input.as_slice()).unwrap();
        assert_eq!(BasicMapping::C, ret_read);
        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(input.to_vec(), ret_write);
    }

    #[test]
    fn test_basic_pattern() {
        let input = [10u8];
        let output = [BasicMapping::C as u8];
        let ret_read = BasicMapping::try_from(input.as_slice()).unwrap();
        assert_eq!(BasicMapping::C, ret_read);
        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(output.to_vec(), ret_write);
    }

    #[test]
    fn test_advanced_remapping() {
        let input = [1u8];
        let output = [1u8];
        let ret_read = AdvancedRemapping::try_from(input.as_slice()).unwrap();
        assert_eq!(AdvancedRemapping::A, ret_read);
        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(output.to_vec(), ret_write);
    }

    #[test]
    fn test_advanced_remapping_default_field() {
        let input = [10u8];
        let output = [3u8];
        let ret_read = AdvancedRemapping::try_from(input.as_slice()).unwrap();
        assert_eq!(AdvancedRemapping::C, ret_read);
        let ret_write: Vec<u8> = ret_read.try_into().unwrap();
        assert_eq!(output.to_vec(), ret_write);
    }
}
