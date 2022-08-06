// fn main() {
//     println!("Hello, world!");
// }

use image::GenericImageView;
//use image::DynamicImage;
//use image::{self, GenericImageView};
use thirtyfour::prelude::*;
//use show_image::event;
use reqwest;
use numpy;
use opencv::{ self as cv, highgui, prelude::*, videoio, Result};
use ndarray::{Array1, ArrayView1, ArrayView3};

//#[show_image::main]
#[tokio::main]
// async fn main() -> WebDriverResult<()> {
    async fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{  

    /* login auth*/
    let username = String::from("admin");
    let password = String::from("admin");

    /* image name */
    let img_name = "cam_img";

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    /* go to camera IP */
    driver.goto("http://192.168.8.155").await?;

    /* Login as user */
    let element = driver.find(By::XPath("//input[@value='Applet' ]")).await?;
    element.click().await?;

    let element = driver.find(By::Name("ID")).await?;
    element.clear().await?;
    element.send_keys(username).await?;

    let element = driver.find(By::Name("PassWord")).await?;
    element.clear().await?;
    element.send_keys(password).await?;

    let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table//tbody/tr/td/table/tbody/tr/td/p/font/input")).await?;
    element.click().await?;

    /* go to camera settings */
    driver.goto("http://192.168.8.155/sysconfig.cgi").await?;

    /* enable image on local network */
    let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/table/tbody/tr/td/input[@name='access_image' and @value=1]")).await?;
    element.click().await?;

    /* set image name */
    let element = driver.find(By::Name("img_name_drt_acs")).await?;
    element.clear().await?;
    element.send_keys(img_name).await?;

    /* submit changes */
    let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/input[@value='Submit']")).await?;
    element.click().await?;

    /* go to image page */
    //driver.goto(format!("http://192.168.8.155/jpg/{}{}", img_name, ".jpg")).await?;
    //let client = reqwest::Client::new();

    // Always explicitly close the browser.
    driver.quit().await?;

    // Create a window with default options and display the image.
    //let window = show_image::create_window("image", Default::default())?;

    let mut counter = 0;

    loop{
        let img_bytes =  reqwest::get(format!("http://192.168.8.155/jpg/{}{}", img_name, ".jpg"))
                                .await?.
                                bytes()
                                .await?;

        let img = image::load_from_memory(&img_bytes)?;
        //img.save(format!("{}{}", img_name, ".jpg"))?;

        let img_arr = ndarray::ArrayView3::from_shape((img.width() as usize, img.height() as usize, 3), img.as_bytes());
        //let img_arr = ndarray::ArrayView3::from_shape((640, 480, 3), &img_bytes);
        // let test = img_arr.unwrap().as_slice();

        let mut img_mat = cv::core::Mat::default();
        unsafe {img_mat.create_rows_cols(480, 640, opencv::imgproc::COLOR_BGR2RGBA)?};
        // unsafe {img_mat.create_rows_cols(img.height() as i32, img.width() as i32, opencv::imgproc::COLOR_BGR2RGBA)?};
        //unsafe {img_mat.cre (img.height() as i32, img.width() as i32, opencv::imgproc::COLOR_BGR2RGBA)?};
        // unsafe {cv::core::Mat::new_rows_cols_with_data( img.height() as i32,
        //                                                 img.width() as i32,
        //                                                 i32::typ(),
        //                                                 &img as std::os::raw::c_void,
        //                                                 cv::core::Mat_AUTO_STEP);}

        
        // let mut img_mat = Mat((img.height(), img.width()), 3, &img_arr);
        // let mut img_mat = cv::core::Mat(10, 20, 3, &img_arr);
        // let img_mat = cv::core::Mat::from_slice(im);

        // img_mat = cv::imgcodecs::imdecode(&test, -1);
        //let img_mat = cv::imgcodecs::imread(img.as_flat_samples_u8().unwrap(), cv::imgcodecs::IMREAD_COLOR);
        
        cv::highgui::imshow("image", &img_mat);
        cv::highgui::wait_key(1);

        counter += 1;
        println!("counter = {}",counter);

    }

    println!("finished");
   
    Ok(())
}
