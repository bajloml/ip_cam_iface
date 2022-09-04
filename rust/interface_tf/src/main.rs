use thirtyfour_sync::prelude::*;
use reqwest;
// use opencv::{ self as cv, prelude::*};
// use opencv::{highgui, imgproc};
// use opencv::core::{Scalar, Vec4b};
// use opencv::prelude::*;

use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use tensorflow::Code;
use tensorflow::Graph;
use tensorflow::SavedModelBundle;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Status;
use tensorflow::Tensor;
use tensorflow::DEFAULT_SERVING_SIGNATURE_DEF_KEY;

use tensorflow::eager::{self, raw_ops, ToTensorHandle};


fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{


    // /* login auth*/
    let _username = String::from("admin");
    let _password = String::from("admin");

    /* image name */
    let _img_name = "cam_img";

    /* camera configuration and setup */
    /* start chromedriver */
    let output = std::process::Command::new("chromedriver")
                                            .output()
                                            .expect("Failed to start process -> chromedrive");

    let caps = DesiredCapabilities::chrome();
    // let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let driver = WebDriver::new("http://localhost:9515", caps)?;

    /* go to camera IP */
    // driver.goto("http://192.168.8.155").await?
    driver.get("http://192.168.8.155")?;

    /* Login as user */
    // let element = driver.find(By::XPath("//input[@value='Applet' ]")).await?;
    let element = driver.find_element(By::XPath("//input[@value='Applet' ]"))?;
    // element.click().await?;
    element.click()?;

    // let element = driver.find(By::Name("ID")).await?;
    let element = driver.find_element(By::Name("ID"))?;
    // element.clear().await?;
    element.clear()?;
    // element.send_keys(_username).await?;
    element.send_keys(_username)?;

    // let element = driver.find(By::Name("PassWord")).await?;
    let element = driver.find_element(By::Name("PassWord"))?;
    // element.clear().await?;
    element.clear()?;
    // element.send_keys(_password).await?;
    element.send_keys(_password)?;

    // let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table//tbody/tr/td/table/tbody/tr/td/p/font/input")).await?;
    let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table//tbody/tr/td/table/tbody/tr/td/p/font/input"))?;
    // element.click().await?;
    element.click()?;

    /* go to camera settings */
    // driver.goto("http://192.168.8.155/sysconfig.cgi").await?;
    driver.get("http://192.168.8.155/sysconfig.cgi")?;

    /* enable image on local network */
    // let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/table/tbody/tr/td/input[@name='access_image' and @value=1]")).await?;
    let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/table/tbody/tr/td/input[@name='access_image' and @value=1]"))?;
    // element.click().await?;
    element.click()?;

    /* set image name */
    // let element = driver.find(By::Name("img_name_drt_acs")).await?;
    let element = driver.find_element(By::Name("img_name_drt_acs"))?;
    // element.clear().await?;
    element.clear()?;
    // element.send_keys(_img_name).await?;
    element.send_keys(_img_name)?;

    /* submit changes */
    // let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/input[@value='Submit']")).await?;
    let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/input[@value='Submit']"))?;
    // element.click().await?;
    element.click()?;

    // Always explicitly close the browser.
    // driver.quit().await?;
    // driver.quit();
    // driver.close()?;
    match driver.close(){
        Ok(res) => println!("Camera configuration is done: {:?}", res),
        // Err(error) => panic!("Problem with closing driver: {:?}", error),
        Err(error) => println!("Problem with closing driver: {:?}", error),
    };

    /* create a client */
    // let client = reqwest::Client::builder().build()?;
    let client = reqwest::blocking::Client::builder().build()?;
    let mut counter = 0 as u32;

    // Load the model.
    let mut workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    workspace.push("models/ssd_mobilenet_v2_320x320_coco17_tpu-8/saved_model");
    let model_path = workspace.display().to_string();
    print!("Loading a model: {}", model_path);
    let start_time = std::time::Instant::now();

    let mut graph = Graph::new();
    let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_path)?;
    let session = &bundle.session;
    println!("model loaded in {:?}", start_time.elapsed());


    // loop{
    //     /* measure time */
    //     let start_time = std::time::Instant::now(); 
                                            
    //     /* get image from url */
    //     // let img_bytes = client.get(format!("http://192.168.8.155/jpg/{}{}", _img_name, ".jpg")).send().await?.bytes().await?;
    //     // let img_bytes = client.get(format!("http://192.168.8.155/jpg/{}{}", _img_name, ".jpg")).send().bytes()?;
    //     let img_bytes = client.get(format!("http://192.168.8.155/jpg/{}{}", _img_name, ".jpg")).send().unwrap().bytes().unwrap();

    //     let img = image::load_from_memory_with_format(img_bytes.as_ref(), image::ImageFormat::Jpeg)?;
    //     // let img = image::load_from_memory(img_bytes.as_ref())?;

    //     /* create cv Mat from the image */
    //     let mut img_mat = Mat::new_rows_cols_with_default(img.height().try_into()?, img.width().try_into()?, opencv::core::CV_8UC3, Scalar::all(0.))?;
    //     // let mut img_mat = Mat::new_rows_cols_with_default(img.height().try_into()?, img.width().try_into()?, cv::core::Vec3b::typ(), cv::core::Scalar::all(0.))?;
        
    //     /* copy image to the cv Mat */
    //     img_mat.data_bytes_mut()?.copy_from_slice(img.as_bytes());

    //     /* switch colors to RGB */
    //     let mut rgb_img_mat = Mat::default();
    //     imgproc::cvt_color(&img_mat, &mut rgb_img_mat, opencv::imgproc::COLOR_RGBA2BGR, 0)?;
        
    //     /* show image */
    //     highgui::imshow("image", &rgb_img_mat)?;
    //     highgui::wait_key(1)?;

    //     counter += 1;
    //     println!("counter = {}, elapsed time = {:?}", counter, start_time.elapsed());

    // }

    println!("finished");

    Ok(())
}
