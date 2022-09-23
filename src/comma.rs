#[derive(Clone, Copy, Default)]
pub(crate) struct ShouldWriteComma(pub(crate) bool);

pub(crate) struct WroteAnything(pub(crate) bool);

impl std::ops::BitOrAssign<WroteAnything> for ShouldWriteComma {
    #[inline]
    fn bitor_assign(&mut self, rhs: WroteAnything) {
        self.0 |= rhs.0;
    }
}
