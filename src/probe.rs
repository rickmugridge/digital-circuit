use crate::device::{Device, DeviceId};
use crate::network::Network;
use crate::wire::{WireId, EventualWireUpdate};

pub struct Probe {
    id: DeviceId,
    in_wire: WireId,
}

impl Probe {
    pub fn new(in_wire: WireId) -> Self {
        Self { id: DeviceId::default(), in_wire }
    }
}

impl Device for Probe {
    fn id(&self) -> DeviceId {
        self.id
    }

    fn set_id(&mut self, id: DeviceId) {
        self.id = id;
    }

    fn in_wires(&self) -> Vec<WireId> {
        vec![self.in_wire]
    }

    fn plan_updates(&self, network: &Network, time: usize) -> Vec<EventualWireUpdate> {
        let wire = network.get_wire(self.in_wire);
        println!("Probe {} of value {} at {time}", wire.name, wire.value);
        vec![]
    }
}