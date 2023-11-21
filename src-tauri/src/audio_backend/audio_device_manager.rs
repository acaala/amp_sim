use anyhow::Error;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, DevicesError,
};

pub struct AudioDeviceManager {
    pub input_device: Device,
    output_device: Device,
}

impl AudioDeviceManager {
    pub fn new() -> Self {
        let host = cpal::default_host();

        // If has config available use that else:
        let input_device = host
            .default_input_device()
            .expect("to have default input device");
        let output_device = host
            .default_output_device()
            .expect("to have default output device");
        AudioDeviceManager {
            input_device,
            output_device,
        }
    }

    pub fn get_input_devices() -> Option<Vec<String>> {
        let host = cpal::default_host();

        let devices = match host.input_devices() {
            Ok(devices) => devices.collect::<Vec<_>>(), // Store devices in a Vec
            Err(e) => {
                println!("Error getting input devices: {:?}", e);
                return None;
            }
        };

        let device_names: Vec<String> = devices.iter().filter_map(|dev| dev.name().ok()).collect();
        Some(device_names)
    }

    pub fn get_output_devices() -> Option<Vec<String>> {
        let host = cpal::default_host();

        let devices = match host.output_devices() {
            Ok(devices) => devices.collect::<Vec<_>>(), // Store devices in a Vec
            Err(e) => {
                println!("Error getting output devices: {:?}", e);
                return None;
            }
        };

        let device_names: Vec<String> = devices.iter().filter_map(|dev| dev.name().ok()).collect();
        Some(device_names)
    }

    pub fn set_input_device(&mut self, new_device: String) -> Result<(), String> {
        let host = cpal::default_host();

        let devices = match host.input_devices() {
            Ok(devices) => devices.collect::<Vec<_>>(), // Store devices in a Vec
            Err(e) => {
                println!("Error getting input devices: {:?}", e);
                return Err(e.to_string());
            }
        };

        let input_device = devices
            .into_iter()
            .find(|x| x.name().unwrap() == new_device);

        match input_device {
            Some(device) => {
                self.input_device = device;
                Ok(())
            }
            None => Err("Failed to get specified device".to_string()),
        }
    }
    pub fn set_output_device(&mut self, new_device: String) -> Result<(), String> {
        let host = cpal::default_host();

        let devices = match host.output_devices() {
            Ok(devices) => devices.collect::<Vec<_>>(), // Store devices in a Vec
            Err(e) => {
                println!("Error getting input devices: {:?}", e);
                return Err(e.to_string());
            }
        };

        let output_device = devices
            .into_iter()
            .find(|x| x.name().unwrap() == new_device);

        match output_device {
            Some(device) => {
                self.input_device = device;
                Ok(())
            }
            None => Err("Failed to get specified device".to_string()),
        }
    }
}
