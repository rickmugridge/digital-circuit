use crate::agenda::{Agenda};
use crate::wire::{Wire, WireId, WireUpdate};
use crate::device::{Device, DeviceId};

pub struct Network {
    wires: Vec<Wire>,
    devices: Vec<Box<dyn Device>>,
    devices_per_wire: Vec<Vec<DeviceId>>,
    agenda: Agenda,
}

impl Network {
    pub fn new() -> Self {
        Self {
            wires: vec![],
            devices: vec![],
            devices_per_wire: vec![],
            agenda: Agenda::new(),
        }
    }

    pub fn add_wire(&mut self, name: String) -> WireId {
        let wire = Wire::new(name);
        self.wires.push(wire);
        self.devices_per_wire.push(vec![]);
        let index = self.wires.len() - 1;
        WireId(index)
    }

    pub fn add_device(&mut self, mut device: Box<dyn Device>) -> DeviceId {
        device.in_wires().iter()
            .for_each(|wire_id| self.devices_per_wire[wire_id.0].push(device.id()));
        let id = DeviceId(self.devices_per_wire.len() - 1);
        device.set_id(id);
        self.devices.push(device);
        id
    }

    fn auto_update_wire(&mut self, wire_update: WireUpdate) {
        let wire_id = wire_update.wire;
        if self.wires[wire_id.0].updated(wire_update.new_value) {
            self.devices_per_wire[wire_id.0].clone().iter().for_each(|i|
                self.devices[i.0].plan_updates(&self, wire_update.time)
                    .into_iter().for_each(|update| { self.agenda.add(update); })
            );
        }
    }

    pub fn update_wire_immediately(&mut self, wire_id: WireId, value: bool) {
        self.auto_update_wire(
            WireUpdate::new(wire_id, self.agenda.current_time, value))
    }

    pub fn get_wire(&self, speaker: WireId) -> &Wire {
        &self.wires[speaker.0]
    }

    pub fn run(&mut self) {
        while let Some(wire_update) = self.agenda.pop() {
            self.auto_update_wire(wire_update)
        }
    }
}
