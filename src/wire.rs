use std::cmp::Ordering;

pub struct Wire {
    pub value: bool,
    pub name: String,
}

impl Wire {
    pub fn new(name: String) -> Self { Self { name, value: false } }

    pub fn updated(&mut self, value: bool) -> bool {
        let updated = self.value != value;
        self.value = value;
        updated
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WireId(pub usize);

impl WireId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

pub struct EventualWireUpdate {
    pub wire: WireId,
    pub delay: usize,
    pub eventual_value: bool,
}

impl EventualWireUpdate {
    pub fn new(wire: WireId,
               delay: usize,
               eventual_value: bool) -> Self {
        Self { wire, delay, eventual_value }
    }
}

#[derive(Debug)]
pub struct WireUpdate {
    pub wire: WireId,
    pub time: usize,
    pub new_value: bool,
}

impl WireUpdate {
    pub fn new(wire: WireId, time: usize, new_value: bool) -> Self {
        Self { wire, time, new_value }
    }
}

impl PartialEq<Self> for WireUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for WireUpdate {}

impl Ord for WireUpdate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for WireUpdate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.time.cmp(&other.time))
    }
}
