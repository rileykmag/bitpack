/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    //check if n fits into width SIGNED bits (signed can be negative...)
    let base: u64 = 2;
    let max = (base.pow(width.try_into().unwrap())) - 1; //can be max negative and max positive

    //get absolute value of n
    let an = n.abs();

    // check if it fits in the maximum 
    if an as u64 <= max  {
        true
    }
    else{
        false
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    // check if n fits into width UNSIGNED bits (unsigned = 0 or positive)
    //width = width of bit field
    //n = the integer to go into the field
    //bit field integer value= (width^2 -1) = max
    // integer has to be less than width^2 -1
    let base: u64 = 2;
    let max = (base.pow(width.try_into().unwrap())) - 1;
    
    // check if it fits in the maximum
    if n <= max  {
        true
    }
    else{
        false
    }
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bits.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
/// or `None` if the field is impossible
pub fn gets(_word: u64, _width: u64, _lsb: u64) -> Option<i64> {
    None
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bytes.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
///
/// or None
/// if `lsb + width > 64`
///
pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    //lsb->right most of field
    //starting point
    //needs to travel width bits to the left (SHIFT with <<)
    //left most bit is the signed one (0 or 1)

    //checking word

    //x = x | ( y << 2) 
    if width == 0 || width > 64 || lsb + width > 64 {
        None
    }
    else {
        //get the field
        let field = word >> lsb; //shift right by lsb
        //mask the field!!
        let mask = (1 << width) - 1; //create a mask of the width bits
        let result = field & mask; //check the field with the mask w/ add
        Some(result)
    }
    
}

/// Given an unsigned 64-bit `word`, and an unsigned `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the unsigned value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    // check width & lsb constraints
    if width == 0 || width > 64 || lsb + width > 64 {
        None
    }
    else{
        //'mask' again from notes
        let mask = (1 << width) - 1;

        // check if value greater than mask
        if value > mask {
            return None;
        }

        //clear original bits in word and set new bits
        // shift the mask to the left by lsb
        let cleared = word & !(mask << lsb); // use & so bits are cleared with ones that are not in the mask and shift
        let new = cleared | (value << lsb); // use | to help set bits in new word to return (adding in the value now!!)

        Some(new)
    }

}

/// Given an unsigned 64-bit `word`, and a signed `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the signed value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
///
pub fn news(_word: u64, _width: u64, _lsb: u64, _value: i64) -> Option<u64> {

    Some(0)
}
