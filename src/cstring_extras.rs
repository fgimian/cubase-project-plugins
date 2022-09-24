use std::ffi::CString;
use std::fmt;

// Adapted from std::ffi::CStr::from_bytes_until_nul which is currently unstable and doesn't
// provide an implementation for CString.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FromVecUntilNulError(());

impl fmt::Display for FromVecUntilNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "data provided does not contain a nul")
    }
}

pub fn from_vec_until_nul(v: Vec<u8>) -> Result<CString, FromVecUntilNulError> {
    let nul_pos = memchr::memchr(0, &v);
    match nul_pos {
        Some(nul_pos) => {
            let subslice = v[..=nul_pos].to_vec();
            // SAFETY: We know there is a nul byte at nul_pos, so this slice
            // (ending at the nul byte) is a well-formed C string.
            Ok(unsafe { CString::from_vec_with_nul_unchecked(subslice) })
        }
        None => Err(FromVecUntilNulError(())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec_until_nul() {
        let xs = b"hello there\0".to_vec();
        assert_eq!(
            from_vec_until_nul(xs).unwrap(),
            CString::new("hello there").unwrap()
        );

        let xs = b"hello\0there".to_vec();
        assert_eq!(
            from_vec_until_nul(xs).unwrap(),
            CString::new("hello").unwrap()
        );

        let xs = b"hello\0there\0".to_vec();
        assert_eq!(
            from_vec_until_nul(xs).unwrap(),
            CString::new("hello").unwrap()
        );

        let xs = b"hello there".to_vec();
        assert!(from_vec_until_nul(xs).is_err());
    }
}
