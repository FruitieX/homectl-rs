use super::{
    device::Device,
    events::{Message, TxEventChannel},
};

pub struct RulesEngine {
    sender: TxEventChannel,
}

impl RulesEngine {
    pub fn new(sender: TxEventChannel) -> Self {
        RulesEngine { sender }
    }

    pub fn device_updated(&self, old: Option<Device>, new: Device) {
        println!("device_updated {:?} (was: {:?})", new, old);

        // TODO: decide whether to emit SetDeviceState based on rules
        if old.is_some() {
            self.sender.send(Message::SetDeviceState(new)).unwrap();
        }
    }
}