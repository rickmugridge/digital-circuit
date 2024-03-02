mod wire;
mod network;
mod device;
mod and_device;
mod or_device;
mod probe;
mod agenda;

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::and_device::AndDevice;
    use crate::network::Network;
    use crate::or_device::OrDevice;
    use crate::probe::Probe;

    #[test]
    fn and() {
        let mut network = Network::new();
        let in1 = network.add_wire("in1".to_string());
        let in2 = network.add_wire("in2".to_string());
        let out = network.add_wire("out".to_string());
        let _and = network.add_device(Box::new(AndDevice::new(in1, in2, out)));
        let _probe = network.add_device(Box::new(Probe::new(out)));
        network.update_wire_immediately(in1, true);
        network.update_wire_immediately(in2, true);
        network.run();
        assert_eq!(network.get_wire(in1).value, true);
        assert_eq!(network.get_wire(out).value, true);
        network.update_wire_immediately(in1, false);
        network.run();
        assert_eq!(network.get_wire(in1).value, false);
        assert_eq!(network.get_wire(out).value, false);
    }

    #[test]
    fn or() {
        let mut network = Network::new();
        let in1 = network.add_wire("in1".to_string());
        let in2 = network.add_wire("in2".to_string());
        let out = network.add_wire("out".to_string());
        let _or = network.add_device(Box::new(OrDevice::new(in1, in2, out)));
        let _probe = network.add_device(Box::new(Probe::new(out)));
        network.update_wire_immediately(in1, true);
        network.run();
        assert_eq!(network.get_wire(out).value, true);
        network.update_wire_immediately(in1, false);
        network.run();
        assert_eq!(network.get_wire(out).value, false);
    }
}

