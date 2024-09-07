#[derive(Debug, PartialEq, Eq)]
pub enum Protocol {
    Udp,
    Tcp
}

impl Protocol {

    pub fn from_value(value: &str) -> Result<Self, ()> {
        for _type in [Self::Udp, Self::Tcp] {
            if _type.value().eq(value) {
                return Ok(_type);
            }
        }

        Err(())
    }

    pub fn value(&self) -> &str {
        match self {
            Self::Udp => "UDP",
            Self::Tcp => "TCP"
        }
    }
}
