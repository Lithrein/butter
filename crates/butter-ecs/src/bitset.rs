pub(crate) trait Bitset {
    fn set_bit(&mut self, nth: usize);
    fn unset_bit(&mut self, nth: usize);
    fn bit(&self, nth: usize) -> bool;
}

impl Bitset for [u64; 1024] {
    fn set_bit(&mut self, nth: usize) {
        let word_index = nth >> 6;
        let bit = nth & 63;
        self[word_index] |= 1 << bit;
    }

    fn unset_bit(&mut self, nth: usize) {
        let word_index = nth >> 6;
        let bit = nth & 63;
        self[word_index] &= !(1 << bit);
    }

    fn bit(&self, nth: usize) -> bool {
        let word_index = nth >> 6;
        let bit = nth & 63;
        (self[word_index] >> bit) & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitset_set() {
        let mut bitset = [0u64; 1024];
        bitset.set_bit(5);
        assert_eq!(bitset[0], 32);

        bitset.set_bit(0);
        assert_eq!(bitset[0], 33);

        bitset.set_bit(64);
        assert_eq!(bitset[1], 1);
    }

    #[test]
    fn bitset_bit() {
        let mut bitset = [0u64; 1024];
        bitset.set_bit(5);
        assert!(bitset.bit(5));
        bitset.set_bit(65);
        assert!(bitset.bit(65));
    }

    #[test]
    fn bitset_unset() {
        let mut bitset = [0u64; 1024];
        bitset.set_bit(3);
        assert!(bitset.bit(3));
        bitset.unset_bit(3);
        assert!(!bitset.bit(3));
    }
}
