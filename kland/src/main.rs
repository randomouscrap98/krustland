use warp::Filter;
use std::error::Error;
use serde::Deserialize;
use s3::{ bucket::Bucket, creds::Credentials, region::Region }; //, S3Error };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let settings = get_settings()?;
   let bucket = make_bucket(&settings)?;
   let rbucket = &bucket; //immutable reference, ownership not transferred

   let s3image = warp::path!("i" / String)
       .and_then(move |imagename| async move {
           let bucketresult = rbucket.get_object(&imagename).await;
           match bucketresult {
               Ok((data, _code)) => {
                   Ok(format!("You asked for: {}, was {} bytes", imagename, data.len()))
               },
               Err(_e) => Err(warp::reject::not_found())
           }
       });

   let routes = s3image.with(warp::cors().allow_any_origin());

   let addr = if settings.hostglobal { [0,0,0,0] } else { [127,0,0,1] };

   println!("Running on {:?}:{}", addr, settings.port);

   warp::serve(routes)
       .run((addr, settings.port))
       .await;

   Ok(())
}

//fn with_imagedata(bucket: &Bucket, name: String) -> impl Filter<Extract = (Vec<u8>,), Error = Infallible> + Clone {
//    warp::any().map(move || {
//
//    })
//}

#[derive(Debug, Deserialize)]
pub struct MySettings {
    pub klanddb : String, 
    pub klandstatic : String,
    pub hostglobal : bool,
    pub s3accesskey : String,
    pub s3secretkey : String,
    pub s3bucket : String,
    pub s3region : String,
    pub port : u16 
}

// Retrieve the settings from the default toml file in the format of "MySettings"
fn get_settings() -> Result<MySettings, config::ConfigError> {
   let mut settings = config::Config::default();
   settings.merge(config::File::with_name("Settings"))?;
   let myset : MySettings = settings.try_into()?;
   Ok(myset) //transfer ownership
}

// Retrieve a configured bucket for connecting to kland images
fn make_bucket(settings: &MySettings) -> Result<Bucket, Box<dyn Error>> {
    let credentials: Credentials = Credentials::from_env_specific(
        Some(&settings.s3accesskey), Some(&settings.s3secretkey), None, None)?;
    let region: Region = settings.s3region.parse()?;
    let bucket: Bucket = Bucket::new(&settings.s3bucket, region, credentials)?; 
    Ok(bucket)
}
