use crate::network::{Network};
use crate::wire::{WireId, EventualWireUpdate};

pub trait Device {
    fn id(&self) -> DeviceId;
    fn set_id(&mut self, id: DeviceId);
    fn in_wires(&self) -> Vec<WireId>;
    fn plan_updates(&self, network: &Network, time:usize) -> Vec<EventualWireUpdate>;
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceId(pub usize);

impl DeviceId {
    pub fn new(i: usize) -> Self {
        Self(i)
    }

    pub fn default() -> Self {
        Self(0)
    }
}
