// fn main() {
//     println!("Hello, world!");
// }

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {

    /* login auth*/
    let username = String::from("admin");
    let password = String::from("admin");

    /* image name */
    let img_name = String::from("cam_img");

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

    let element = driver.find(By::Name("img_name_drt_acs")).await?;
    element.clear().await?;
    element.send_keys(img_name).await?;

    let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/input[@value='Submit']")).await?;
    element.click().await?;
   
    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
