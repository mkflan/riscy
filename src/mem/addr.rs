#[derive(Debug, Clone, Copy)]
pub struct VirtAddr(usize);

impl VirtAddr {
    #[inline]
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    /// Return the page offset of this virtual address.
    pub fn offset(self) -> usize {
        // The page offset is stored in the bottom 12 bits of the virtual address.
        self.0 & 0xFFF
    }

    /// Return the desired virtual page number.
    pub fn vpn(self, idx: u8) -> usize {
        assert!(idx >= 0 && idx <= 4);

        (self.0 >> (12 + 9 * idx)) & 0x1FF
    }
}
