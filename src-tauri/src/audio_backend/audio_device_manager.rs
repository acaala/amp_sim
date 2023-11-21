use anyhow::Error;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device,
};

pub struct AudioDeviceManager {
    pub input_device: Device,
    pub output_device: Device,
}

impl AudioDeviceManager {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let input_device = host
            .default_input_device()
            .expect("To get default input device");
        let output_device = host
            .default_output_device()
            .expect("To get default output device");

        AudioDeviceManager {
            input_device,
            output_device,
        }
    }

    pub fn get_input_devices() -> Result<Vec<String>, Error> {
        let host = cpal::default_host();

        let devices = host.input_devices()?;
        let device_names: Vec<String> = devices.filter_map(|dev| dev.name().ok()).collect();

        Ok(device_names)
    }

    pub fn get_output_devices() -> Result<Vec<String>, Error> {
        let host = cpal::default_host();

        let devices = host.output_devices()?;
        let device_names: Vec<String> = devices.filter_map(|dev| dev.name().ok()).collect();

        Ok(device_names)
    }

    pub fn set_input_device(&mut self, new_device_name: String) -> Result<(), Error> {
        let host = cpal::default_host();

        let devices = host.input_devices()?;
        let input_device = devices
            .into_iter()
            .find(|x| x.name().unwrap() == new_device_name);

        self.input_device = input_device.unwrap();

        Ok(())
    }

    pub fn set_output_device(&mut self, new_device_name: String) -> Result<(), Error> {
        let host = cpal::default_host();

        let devices = host.output_devices()?;
        let output_device = devices
            .into_iter()
            .find(|x| x.name().unwrap() == new_device_name);

        self.input_device = output_device.unwrap();

        Ok(())
    }
}
