use crate::device::{Device, DeviceId};
use crate::network::Network;
use crate::wire::{WireId, EventualWireUpdate};

pub struct AndDevice {
    id: DeviceId,
    wire_in1: WireId,
    wire_in2: WireId,
    wire_out: WireId,
}

impl AndDevice {
    pub fn new(wire_in1: WireId, wire_in2: WireId, wire_out: WireId) -> Self {
        Self { id: DeviceId::new(0), wire_in1, wire_in2, wire_out }
    }
    fn compute(&self, network: &Network) -> bool {
        network.get_wire(self.wire_in1).value && network.get_wire(self.wire_in2).value
    }
}

impl Device for AndDevice {
    fn id(&self) -> DeviceId {
        self.id
    }
    fn set_id(&mut self, id: DeviceId) {
        self.id = id;
    }
    fn in_wires(&self) -> Vec<WireId> {
        vec![self.wire_in1, self.wire_in2]
    }
    fn plan_updates(&self, network: &Network, _time: usize) -> Vec<EventualWireUpdate> {
        let eventual_value = self.compute(network);
        vec![EventualWireUpdate::new(self.wire_out, 3, eventual_value)]
    }
}
