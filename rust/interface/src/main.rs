// fn main() {
//     println!("Hello, world!");
// }

use thirtyfour::prelude::*;
use reqwest;
use opencv::{ self as cv, prelude::*};

//#[show_image::main]
#[tokio::main]
// async fn main() -> WebDriverResult<()> {
    async fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{

        /* login auth*/
        let username = String::from("admin");
        let password = String::from("admin");

        /* image name */
        let img_name = "cam_img";

        /* camera configuration and setup */
        {    
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
        }

        let mut counter = 0 as u32;

        loop{
            /* measure time */
            let startTime = std::time::Instant::now(); 
                                                
            /* get image from url */
            let img_bytes =  reqwest::get(format!("http://192.168.8.155/jpg/{}{}", img_name, ".jpg"))
            .await?.
            bytes()
            .await?; 

            let img = image::load_from_memory(img_bytes.as_ref())?;

            /* create cv Mat from the image */
            let mut img_mat = Mat::new_rows_cols_with_default(img.height().try_into()?, img.width().try_into()?, cv::core::CV_8UC3, cv::core::Scalar::all(0.))?;
            // let mut img_mat = Mat::new_rows_cols_with_default(img.height().try_into()?, img.width().try_into()?, cv::core::Vec3b::typ(), cv::core::Scalar::all(0.))?;
            
            /* copy image to the cv Mat */
            img_mat.data_bytes_mut()?.copy_from_slice(img.as_bytes());

            /* switch colors to RGB */
            let mut rgb_img_mat = Mat::default();
            cv::imgproc::cvt_color(&img_mat, &mut rgb_img_mat, cv::imgproc::COLOR_RGBA2BGR, 0)?;
            
            /* show image */
            cv::highgui::imshow("image", &rgb_img_mat)?;
            cv::highgui::wait_key(1)?;

            counter += 1;
            println!("counter = {}, elapsed time = {:?}", counter, startTime.elapsed());

        }

        println!("finished");
    
        Ok(())
    }
