use warp::Filter;
use std::error::Error;
use serde::Deserialize;
use s3::{ bucket::Bucket, credentials::Credentials, region::Region }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let settings = get_settings()?;

    let s3image = warp::path!("i" / String)
        .map(|imagename| {
            format!("You asked for: {}", imagename)
        });
    
    let addr = if settings.hostglobal { [0,0,0,0] } else { [127,0,0,1] };

    println!("Running on {:?}:{}", addr, settings.port);

    warp::serve(s3image)
        .run((addr, settings.port))
        .await;
    
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct MySettings {
    pub klanddb : String, 
    pub klandstatic : String,
    pub hostglobal : bool,
    pub port : u16 
}

// Retrieve the settings from the default toml file in the format of "MySettings"
fn get_settings() -> Result<MySettings, config::ConfigError> {
   let mut settings = config::Config::default();
   settings.merge(config::File::with_name("Settings"))?;
   let myset : MySettings = settings.try_into()?;
   Ok(myset) //transfer ownership
}

