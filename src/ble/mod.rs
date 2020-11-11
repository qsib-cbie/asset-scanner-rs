extern crate btleplug;

use crate::error::*;

use async_std::{prelude::StreamExt, sync::Sender, task};
// use std::fs::OpenOptions;
// use std::io::prelude::*;

use btleplug::api::{BDAddr, Central, CentralEvent, Peripheral, UUID};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::ConnectedAdapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};

// adapter retrieval works differently depending on your platform right now.
// API needs to be aligned.

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub struct CentralManager {
    central: Adapter,
}

#[cfg(target_os = "linux")]
fn get_central(manager: &Manager) -> ConnectedAdapter {
    let adapters = manager.adapters().unwrap();
    let adapter = adapters.into_iter().nth(0).unwrap();
    adapter.connect().unwrap()
}

#[cfg(target_os = "linux")]
pub struct CentralManager {
    central: ConnectedAdapter,
}

pub struct Context {
    manager: Manager,
}

impl Context {
    pub fn new() -> Context {
        Context {
            manager: Manager::new().unwrap(),
        }
    }
}

impl CentralManager {
    pub fn new(context: &Context) -> CentralManager {
        CentralManager {
            central: get_central(&context.manager),
        }
    }

    fn handle_discovered(
        self: &Self,
        _sender: Sender<EventMessage>,
        bd_addr: BDAddr,
    ) -> Result<()> {
        log::trace!("DeviceDiscovered: {:#?}", bd_addr);
        let peripheral = self.central.peripheral(bd_addr);
        log::trace!("Peripheral: {:#?}", peripheral);
        match peripheral {
            Some(peripheral) => match peripheral.properties().local_name {
                Some(name) => {
                    if name.starts_with("QSIB") {
                        log::debug!("DeviceDiscovered: {:#?}", bd_addr);
                        log::debug!("Found {} with {:?}", name, peripheral.properties());
                        peripheral.connect()?;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    fn handle_updated(self: &Self, _sender: Sender<EventMessage>, bd_addr: BDAddr) -> Result<()> {
        log::info!("DeviceUpdated {:?}", bd_addr);
        let peripheral = self.central.peripheral(bd_addr);
        match peripheral {
            Some(peripheral) => match peripheral.properties().local_name {
                Some(name) => {
                    if name.starts_with("QSIB") {
                        log::debug!("Updated {} with {:?}", name, peripheral.properties());
                        for c in peripheral.characteristics() {
                            log::debug!("Updated Characteristic: {:#?}", c);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    fn handle_connected(self: &Self, sender: Sender<EventMessage>, bd_addr: BDAddr) -> Result<()> {
        log::info!("DeviceConnected: {:?}", bd_addr);
        let peripheral = self.central.peripheral(bd_addr);
        match peripheral {
            Some(peripheral) => {
                log::debug!(
                    "Connected {:?} with {:?}",
                    peripheral.properties().local_name,
                    peripheral.properties()
                );

                let value_peripheral_clone = peripheral.clone();
                let value_sender = sender.clone();
                let value_char_uuid: UUID =
                    "85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99".parse()?;
                peripheral.on_discovery(
                    value_char_uuid,
                    Box::new(move |discovery| {
                        log::debug!("UUID {:?} Discovery: {:#?}", value_char_uuid, discovery);
                        let result = || -> Result<()> {
                            let value = value_peripheral_clone.read(&discovery)?;
                            let value = String::from_utf8(value)?;
                            task::block_on(async {
                                value_sender
                                    .send(EventMessage::CoalesceEvent(value_char_uuid, value))
                                    .await;
                            });
                            Ok(())
                        }();

                        match result {
                            Ok(_) => {}
                            Err(err) => {
                                log::error!(
                                    "Failed to do something with characteristic discovery: {:?}",
                                    err
                                );
                            }
                        }
                    }),
                );

                let data_peripheral_clone = peripheral.clone();
                let data_sender = sender.clone();
                let data_char_uuid: UUID =
                    "97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99".parse()?;
                peripheral.on_discovery(data_char_uuid, Box::new(move |discovery| {
                    log::debug!("UUID {:?} Discovery: {:#?}", data_char_uuid, discovery);
                    let result = || -> Result<()> {
                        let value = data_peripheral_clone.read(&discovery)?;
                        let value = String::from_utf8(value)?;
                        task::block_on(async {
                            data_sender.send(EventMessage::CoalesceEvent(data_char_uuid, value)).await;
                        });
                        Ok(())
                    }();

                    match result {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("Failed to do something with the discovered characteristics: {:?}", err);
                        }

                    }
                }));

                for c in peripheral.discover_characteristics()? {
                    log::debug!("Already have Characteristic: {:#?}", c);
                }

                task::block_on(async {
                    sender
                        .send(EventMessage::PendingDisconnect(
                            std::time::Duration::from_secs(10),
                            peripheral.clone(),
                        ))
                        .await
                });

                task::block_on(async {
                    sender
                        .send(EventMessage::PendingConnect(
                            std::time::Duration::from_secs(20),
                            peripheral.clone(),
                        ))
                        .await
                });
            }
            _ => log::info!("PERIPHERAL IS GONE!!!!!"),
        }

        Ok(())
    }

    fn handle_disconnected(
        self: &Self,
        _sender: Sender<EventMessage>,
        bd_addr: BDAddr,
    ) -> Result<()> {
        log::info!("DeviceDisconnected: {:#?}", bd_addr);

        Ok(())
    }

    fn handle_lost(self: &Self, _sender: Sender<EventMessage>, bd_addr: BDAddr) -> Result<()> {
        log::trace!("DeviceLost: {:#?}", bd_addr);
        Ok(())
    }

    pub fn run(self) -> Result<()> {
        let event_receiver = self.central.event_receiver().unwrap();
        self.central.start_scan()?;

        let (sender, receiver) = async_std::sync::channel::<EventMessage>(256);
        let sender_clone = sender.clone();
        task::spawn(async move {
            loop {
                task::sleep(std::time::Duration::from_secs(1)).await;
                log::trace!("Tik ...");
                sender_clone.send(EventMessage::TikTok).await;
            }
        });

        let sender_clone = sender.clone();
        task::spawn(async move {
            let mut coalescing_start: Option<std::time::Instant> = None;
            let mut coalescing_payload = std::collections::HashSet::new();
            loop {
                let sender = sender_clone.clone();
                let mut receiver = receiver.clone();
                let event_message: EventMessage =
                    task::block_on(async { receiver.next().await.unwrap() });
                match event_message {
                    EventMessage::PendingConnect(duration, peripheral) => {
                        log::info!("Pending connection in {:?}", duration);
                        task::spawn(async move {
                            async_std::task::sleep(duration).await;
                            log::info!(
                                "Attempting connecting {:?}",
                                peripheral.properties().local_name
                            );
                            match peripheral.connect() {
                                Ok(_) => {}
                                Err(err) => {
                                    sender.send(EventMessage::Error(CliError::from(err))).await;
                                }
                            }
                        });
                    }
                    EventMessage::PendingDisconnect(duration, peripheral) => {
                        log::info!("Pending disconnection in {:?}", duration);
                        task::spawn(async move {
                            async_std::task::sleep(duration).await;
                            log::info!("Disconnecting {:?}", peripheral.properties().local_name);
                            match peripheral.disconnect() {
                                Ok(_) => {}
                                Err(err) => {
                                    sender.send(EventMessage::Error(CliError::from(err))).await;
                                }
                            }
                        });
                    }
                    EventMessage::TikTok => {
                        log::trace!("Tok ...");
                        match coalescing_start {
                            Some(start) => {
                                if start.elapsed() > std::time::Duration::from_secs(2) {
                                    log::info!("Mark event: {:?}", coalescing_payload);
                                    coalescing_payload.clear();
                                    coalescing_start = None;
                                }
                            }
                            _ => {}
                        }
                    }
                    EventMessage::CoalesceEvent(char_uuid, message) => {
                        log::info!("Processing {} for {}", message, char_uuid);
                        coalescing_payload.insert(message);
                        match coalescing_start {
                            Some(start) => {
                                if start.elapsed().as_secs() > 5 {
                                    log::info!("Mark event: {:?}", coalescing_payload);
                                    coalescing_payload.clear();
                                    coalescing_start = None;
                                }
                            }
                            None => {
                                coalescing_start = Some(std::time::Instant::now());
                            }
                        }
                    }
                    EventMessage::Error(reason) => {
                        log::error!("Received Err message: {}", reason);
                    }
                }
            }
        });

        loop {
            let event = match event_receiver.recv() {
                Ok(event) => event,
                Err(err) => {
                    log::error!("Failed to recv event: {:?}", err);
                    return Err(CliError::from(err));
                }
            };

            let sender = sender.clone();
            match event {
                CentralEvent::DeviceDiscovered(bd_addr) => {
                    self.handle_discovered(sender, bd_addr)?;
                }
                CentralEvent::DeviceUpdated(bd_addr) => {
                    self.handle_updated(sender, bd_addr)?;
                }
                CentralEvent::DeviceConnected(bd_addr) => {
                    self.handle_connected(sender, bd_addr)?;
                }
                CentralEvent::DeviceDisconnected(bd_addr) => {
                    self.handle_disconnected(sender, bd_addr)?;
                }
                CentralEvent::DeviceLost(bd_addr) => {
                    self.handle_lost(sender, bd_addr)?;
                }
            }
        }
    }
}

pub enum EventMessage {
    TikTok, // nop nop nop
    PendingConnect(
        std::time::Duration,
        btleplug::corebluetooth::peripheral::Peripheral,
    ),
    PendingDisconnect(
        std::time::Duration,
        btleplug::corebluetooth::peripheral::Peripheral,
    ),
    CoalesceEvent(UUID, String),
    Error(CliError),
}

#[cfg(test)]
mod tests {
    use btleplug::api::UUID;

    #[test]
    fn parse_asset_uuids() {
        let value_char_uuid = "85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99";
        let data_char_uuid = "97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99";
        let version_char_uuid = "4C:C6:98:18:27:C5:4D:34:99:36:D4:CC:61:88:EE:99";
        let error_char_uuid = "08:AE:FE:30:27:C5:4D:34:99:36:D4:CC:61:88:EE:99";

        for input in [
            value_char_uuid,
            data_char_uuid,
            version_char_uuid,
            error_char_uuid,
        ]
        .iter()
        {
            let result: Result<UUID, _> = input.parse();

            if let Ok(uuid) = result {
                assert_eq!(String::from(*input), uuid.to_string());
            }
        }
    }
}
