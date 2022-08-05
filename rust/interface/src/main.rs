// fn main() {
//     println!("Hello, world!");
// }

//use image::DynamicImage;
//use image::{self, GenericImageView};
use thirtyfour::prelude::*;
//use show_image::event;
use reqwest;
use opencv::{ self as cv, highgui, prelude::*, videoio, Result};

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

    loop{
        let img_bytes =  reqwest::get(format!("http://192.168.8.155/jpg/{}{}", img_name, ".jpg"))
                                .await?.
                                bytes()
                                .await?;

        let img = image::load_from_memory(&img_bytes)?;
        img.save(format!("{}{}", img_name, ".jpg"))?;
        let test = img.as_bytes();

        let img_mat = cv::imgcodecs::imdecode(img.as_bytes(), cv::imgcodecs::IMREAD_COLOR);
        
        cv::highgui::imshow("image", img_mat);

        std::thread::sleep(std::time::Duration::from_millis(10));

    }

    println!("finished");
   
    Ok(())
}
