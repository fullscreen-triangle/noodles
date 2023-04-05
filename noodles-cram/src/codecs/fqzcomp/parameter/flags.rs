bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Flags: u8 {
        const RESERVED = 0x01;
        const DO_DEDUP = 0x02;
        const DO_LEN = 0x04;
        const DO_SEL = 0x08;
        const HAVE_QMAP = 0x10;
        const HAVE_PTAB = 0x20;
        const HAVE_DTAB = 0x40;
        const HAVE_QTAB = 0x80;
    }
}

impl From<u8> for Flags {
    fn from(n: u8) -> Self {
        Self::from_bits_truncate(n)
    }
}

impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        flags.bits()
    }
}
