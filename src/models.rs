extern crate reqwest;
use std::io::Read;

const BASE_PARTICLE_URL: &str = "https://api.particle.io/v1/";

// General API Structs

pub struct ParticleCloud {
    cloud_api_token: String
}
impl ParticleCloud {

    pub fn new(_token: &str) -> ParticleCloud {
        ParticleCloud {
            cloud_api_token: _token.to_string()
        }
    }

    pub fn get_devices(&self) -> Result<(), Box<dyn std::error::Error>>  {
        let url = BASE_PARTICLE_URL.to_string() + "devices?access_token=" + &self.cloud_api_token;
        let mut request = reqwest::blocking::get(url)?;

        if request.status() == 200 {
            println!("{:?}", request);
            // TODO: Actually get the devices and create device objects from them
        }
    Ok(())
    }
}

pub struct HubManager {
    cloud_api_token: String,
    devices: Vec<Device>,
    state_filename: String,
    log_managers: Vec<LogManager>
}


pub struct Device {

}


pub struct LogManager {

}