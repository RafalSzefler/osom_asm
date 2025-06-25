use core::fmt;

#[derive(PartialEq, Eq)]
pub struct TmpSlice<'a> {
    pub data: &'a [u8],
}

impl fmt::Debug for TmpSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut iter = self.data.iter();
        if let Some(item) = iter.next() {
            write!(f, "{item:#04X}")?;
        }
        for item in iter {
            write!(f, ", {item:#04X}")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

pub trait TmpSlicable {
    fn as_tmp_slice(&self) -> TmpSlice<'_>;
}

impl TmpSlicable for [u8] {
    fn as_tmp_slice(&self) -> TmpSlice<'_> {
        TmpSlice { data: self }
    }
}

impl TmpSlicable for &[u8] {
    fn as_tmp_slice(&self) -> TmpSlice<'_> {
        TmpSlice { data: self }
    }
}

impl TmpSlicable for Vec<u8> {
    fn as_tmp_slice(&self) -> TmpSlice<'_> {
        TmpSlice { data: self.as_slice() }
    }
}

impl<const N: usize> TmpSlicable for [u8; N] {
    fn as_tmp_slice(&self) -> TmpSlice<'_> {
        TmpSlice { data: self }
    }
}

impl<const N: usize> TmpSlicable for &[u8; N] {
    fn as_tmp_slice(&self) -> TmpSlice<'_> {
        TmpSlice { data: *self }
    }
}
