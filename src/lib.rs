pub mod bitpack;
pub use crate::bitpack::fitss;
pub use crate::bitpack::fitsu;
pub use crate::bitpack::gets;
pub use crate::bitpack::getu;
pub use crate::bitpack::news;
pub use crate::bitpack::newu;

#[cfg(test)]
mod tests {
    use crate::bitpack;
    use rand::Rng;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // test for fitss
    #[test]
    fn test_fitss() {
        // check if n fits into width SIGNED bits
        assert_eq!(bitpack::fitss(0, 1), true);
        assert_eq!(bitpack::fitss(1, 1), true);
        assert_eq!(bitpack::fitss(2, 1), false);
        assert_eq!(bitpack::fitss(16, 4), false);
        assert_eq!(bitpack::fitss(15, 4), true);
        assert_eq!(bitpack::fitss(-1, 1), true);
        assert_eq!(bitpack::fitss(-2, 1), false);
    }

    // tests for fitsu
    #[test]
    fn test_fitsu() {
        // check if n fits into width UNSIGNED bits
        assert_eq!(bitpack::fitsu(0, 1), true);
        assert_eq!(bitpack::fitsu(1, 1), true);
        assert_eq!(bitpack::fitsu(2, 1), false);
        assert_eq!(bitpack::fitsu(16, 4), false);
        assert_eq!(bitpack::fitsu(15, 4), true);
    }

    // test for gets
    #[test]
    fn test_gets() {
        // test for none or SIGNED value corresponding to representation of word

        
    }

    // test for getu
    #[test]
    fn test_getu() {
        // test for none or UNSIGNED value corresponding to representation of word
        assert_eq!(bitpack::getu(0b1111_0000, 4, 4), Some(0b1111)); 
        assert_eq!(bitpack::getu(0b1010_1010, 3, 1), Some(0b101)); 
        assert_eq!(bitpack::getu(0, 0, 0), None); 
        assert_eq!(bitpack::getu(0, 65, 0), None);
        assert_eq!(bitpack::getu(0, 4, 61), None); 
      
    }

    // test for news
    #[test]
    fn test_news() {
        // test for desired SIGNED value at appropriate field, or NONE
    }

    // test for newu
    #[test]
    fn test_newu() {
        // test for desired UNSIGNED value at appropriate field, or NONE
        assert_eq!(bitpack::newu(0, 4, 0, 0b1010), Some(0b1010));
        assert_eq!(bitpack::newu(0b1111_0000, 4, 0, 0b1010), Some(0b1111_1010));
        assert_eq!(bitpack::newu(0b1111_0000, 4, 4, 0b1010), Some(0b1010_0000));
        assert_eq!(bitpack::newu(0, 4, 0, 0b10000), None);
        

    }
}
