use crate::wire::{EventualWireUpdate, WireUpdate};

pub struct Agenda {
    pub current_time: usize,
    items: Vec<WireUpdate>,
}

impl Agenda {
    pub fn new() -> Self { Self { current_time: 0, items: vec![] } }

    pub fn add(&mut self, update: EventualWireUpdate) {
        let time = self.current_time + update.delay;
        let agenda_item = WireUpdate {
            time,
            wire: update.wire,
            new_value: update.eventual_value,
        };
        if let Some(i) = self.items.iter().position(|a| a.time < time) {
            self.items.insert(i, agenda_item);
        } else {
            self.items.push(agenda_item);
        }
    }

    pub fn pop(&mut self) -> Option<WireUpdate> {
        if let Some(t) = self.items.pop() {
            self.current_time = t.time;
            Some(t)
        } else { None }
    }
}

#[cfg(test)]
mod tests {
    use crate::wire::WireId;
    use super::*;

    #[test]
    fn is_empty() {
        let mut agenda = Agenda::new();
        assert_eq!(agenda.pop(), None)
    }

    #[test]
    fn pops_earliest_when_added_in_correct_order() {
        let mut agenda = Agenda::new();
        agenda.add(EventualWireUpdate::new(WireId::new(2), 4, true));
        agenda.add(EventualWireUpdate::new(WireId::new(4), 6, true));
        agenda.add(EventualWireUpdate::new(WireId::new(6), 8, true));
        assert_eq!(agenda.pop(), Some(WireUpdate { wire: WireId(2), time: 4, new_value: true }));
        assert_eq!(agenda.pop(), Some(WireUpdate { wire: WireId(4), time: 6, new_value: true }));
        assert_eq!(agenda.pop(), Some(WireUpdate { wire: WireId(6), time: 8, new_value: true }));
        assert_eq!(agenda.pop(), None);
    }

    #[test]
    fn pops_earliest_when_added_out_of_order() {
        let mut agenda = Agenda::new();
        agenda.add(EventualWireUpdate::new(WireId::new(6), 8, true));
        agenda.add(EventualWireUpdate::new(WireId::new(4), 6, true));
        agenda.add(EventualWireUpdate::new(WireId::new(2), 4, true));
        assert_eq!(agenda.pop(), Some(WireUpdate { wire: WireId(2), time: 4, new_value: true }));
        assert_eq!(agenda.pop(), Some(WireUpdate { wire: WireId(4), time: 6, new_value: true }));
        assert_eq!(agenda.pop(), Some(WireUpdate { wire: WireId(6), time: 8, new_value: true }));
        assert_eq!(agenda.pop(), None);
    }

    #[test]
    fn add_after_pop_uses_advanced_time() {
        let mut agenda = Agenda::new();
        agenda.add(EventualWireUpdate::new(WireId::new(6), 8, true));
        assert_eq!(agenda.pop(), Some(WireUpdate { wire: WireId(6), time: 8, new_value: true }));
        assert_eq!(agenda.pop(), None);
        agenda.add(EventualWireUpdate::new(WireId::new(2), 4, true));
        assert_eq!(agenda.pop(), Some(WireUpdate { wire: WireId(2), time: 12, new_value: true }));
        assert_eq!(agenda.pop(), None);
    }
}