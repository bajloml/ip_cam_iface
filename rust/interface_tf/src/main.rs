use tensorflow::eager::raw_ops::random_shuffle_queue;
use thirtyfour_sync::prelude::*;
use reqwest;
// use opencv::{ self as cv, prelude::*};
// use opencv::{highgui, imgproc};
// use opencv::core::{Scalar, Vec4b};
// use opencv::prelude::*;

use image::*;

use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

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

// Make it a bit nicer to work with the results, by adding a more explanatory struct
pub struct BBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub prob: f32,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{

    // /* login auth*/
    let _username = String::from("admin");
    let _password = String::from("admin");

    /* image name */
    let _img_name = "cam_img";

    /* camera configuration and setup */
    // {
    //     /* start chromedriver */
    //     let output = std::process::Command::new("chromedriver")
    //                                             .output()
    //                                             .expect("Failed to start process -> chromedrive");

    //     let caps = DesiredCapabilities::chrome();
    //     // let driver = WebDriver::new("http://localhost:9515", caps).await?;
    //     let driver = WebDriver::new("http://localhost:9515", caps)?;

    //     /* go to camera IP */
    //     // driver.goto("http://192.168.8.155").await?
    //     driver.get("http://192.168.8.155")?;

    //     /* Login as user */
    //     // let element = driver.find(By::XPath("//input[@value='Applet' ]")).await?;
    //     let element = driver.find_element(By::XPath("//input[@value='Applet' ]"))?;
    //     // element.click().await?;
    //     element.click()?;

    //     // let element = driver.find(By::Name("ID")).await?;
    //     let element = driver.find_element(By::Name("ID"))?;
    //     // element.clear().await?;
    //     element.clear()?;
    //     // element.send_keys(_username).await?;
    //     element.send_keys(_username)?;

    //     // let element = driver.find(By::Name("PassWord")).await?;
    //     let element = driver.find_element(By::Name("PassWord"))?;
    //     // element.clear().await?;
    //     element.clear()?;
    //     // element.send_keys(_password).await?;
    //     element.send_keys(_password)?;

    //     // let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table//tbody/tr/td/table/tbody/tr/td/p/font/input")).await?;
    //     let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table//tbody/tr/td/table/tbody/tr/td/p/font/input"))?;
    //     // element.click().await?;
    //     element.click()?;

    //     /* go to camera settings */
    //     // driver.goto("http://192.168.8.155/sysconfig.cgi").await?;
    //     driver.get("http://192.168.8.155/sysconfig.cgi")?;

    //     /* enable image on local network */
    //     // let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/table/tbody/tr/td/input[@name='access_image' and @value=1]")).await?;
    //     let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/table/tbody/tr/td/input[@name='access_image' and @value=1]"))?;
    //     // element.click().await?;
    //     element.click()?;

    //     /* set image name */
    //     // let element = driver.find(By::Name("img_name_drt_acs")).await?;
    //     let element = driver.find_element(By::Name("img_name_drt_acs"))?;
    //     // element.clear().await?;
    //     element.clear()?;
    //     // element.send_keys(_img_name).await?;
    //     element.send_keys(_img_name)?;

    //     /* submit changes */
    //     // let element = driver.find(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/input[@value='Submit']")).await?;
    //     let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/input[@value='Submit']"))?;
    //     // element.click().await?;
    //     element.click()?;

    //     // Always explicitly close the browser.
    //     // driver.quit().await?;
    //     // driver.quit();
    //     // driver.close()?;
    //     match driver.close(){
    //         Ok(res) => println!("Camera configuration is done: {:?}", res),
    //         // Err(error) => panic!("Problem with closing driver: {:?}", error),
    //         Err(error) => println!("Problem with closing driver: {:?}", error),
    //     };
    // }

    /* create a client */
    // let client = reqwest::Client::builder().build()?;
    let client = reqwest::blocking::Client::builder().build()?;
    let mut counter = 0 as u32;

    // Load the model.
    let mut workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    workspace.push("models/ssd_mobilenet_v2_320x320_coco17_tpu-8/saved_model");
    let model_path = workspace.display().to_string();
    print!("Loading a model: {}", model_path);
    
    let model_loading_time = std::time::Instant::now();
    let mut graph = Graph::new();
    let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_path)?;
    let session = &bundle.session;
    println!("model loaded in {:?}", model_loading_time.elapsed());

    // Create an eager execution context
    let opts = eager::ContextOptions::new();
    let ctx = eager::Context::new(opts)?;

    loop{
        /* measure time */
        let start_time = std::time::Instant::now(); 
                                            
        /* get image from url */
        // let img_bytes = client.get(format!("http://192.168.8.155/jpg/{}{}", _img_name, ".jpg")).send().await?.bytes().await?;
        // let img_bytes = client.get(format!("http://192.168.8.155/jpg/{}{}", _img_name, ".jpg")).send().bytes()?;
        let img_bytes = client.get(format!("http://192.168.8.155/jpg/{}{}", _img_name, ".jpg")).send().unwrap().bytes().unwrap();

        let img = image::load_from_memory_with_format(img_bytes.as_ref(), image::ImageFormat::Jpeg)?;

        let img_pixels = img.as_rgb8().unwrap().pixels();

        let mut vec_flattened: Vec<u8> = Vec::new();
        for rgb in img_pixels{
            vec_flattened.push(rgb[2] as u8);
            vec_flattened.push(rgb[1] as u8);
            vec_flattened.push(rgb[0] as u8);
        }

        //The `input` tensor expects BGR pixel data.
        let input = Tensor::new(&[1, img.height() as u64, img.width() as u64, 3]).with_values(&vec_flattened)?;

        // get in/out operations
        let signature = bundle.meta_graph_def().get_signature(DEFAULT_SERVING_SIGNATURE_DEF_KEY)?;

        let input_info = signature.get_input("input_tensor")?;

        let output_info_boxes = signature.get_output("detection_boxes")?;
        let output_info_classes = signature.get_output("detection_classes")?;
        let output_info_scores = signature.get_output("detection_scores")?;

        // Run the graph.
        let mut args = SessionRunArgs::new();
        
        // load input image
        args.add_feed(&graph.operation_by_name_required(&input_info.name().name)?, 0, &input);
        
        // output operations
        let token_output_boxes = args.request_fetch(&graph.operation_by_name_required(&output_info_boxes.name().name)?, 0);
        let token_output_classes = args.request_fetch(&graph.operation_by_name_required(&output_info_classes.name().name)?, 0);
        let token_output_scores = args.request_fetch(&graph.operation_by_name_required(&output_info_scores.name().name)?, 0);

        session.run(&mut args)?;

        // Check the output.
        let output_boxes: Tensor<f32> = args.fetch(token_output_boxes)?;
        let output_classes: Tensor<f32> = args.fetch(token_output_classes)?;
        let output_scores: Tensor<f32> = args.fetch(token_output_scores)?;


        // Let's store the results as a Vec<BBox>
        let bboxes: Vec<_> = output_boxes
        .chunks_exact(4) // Split into chunks of 4
        .zip(output_scores.iter()) // Combine it with prob_res
        .map(|(token_output_boxes, &output_scores)| BBox {
            y1: token_output_boxes[0],
            x1: token_output_boxes[1],
            y2: token_output_boxes[2],
            x2: token_output_boxes[3],
            prob: output_scores,
        })
        .collect();

        println!("BBox Length: {}, BBoxes:{:#?}", bboxes.len(), bboxes);

        //We want to change input_image since it is not needed.
        let mut img_out = img.to_rgb8();

        //Iterate through all bounding boxes
        for bbox in bboxes {

            //Create a `Rect` from the bounding box.
            let rect = Rect::at(bbox.x1 as i32, bbox.y1 as i32)
                .of_size((bbox.x2 - bbox.x1) as u32, (bbox.y2 - bbox.y1) as u32);

            //Draw a green line around the bounding box
            draw_hollow_rect_mut(&mut img_out, rect, image::Rgba([0, 255, 0, 0]));
        }

        //Once we've modified the image we save it in the output location.
        img_out.save(&opt.output)?;

        print!("test");

        // Load an input image.
        // let fname = "examples/mobilenetv3/sample.png".to_handle(&ctx)?;
        // let buf = raw_ops::read_file(&ctx, &fname)?;
        // let img = raw_ops::decode_jpeg(&ctx, &buf)?;
        // let img = raw_ops::decode_image(&ctx, &buf)?;
        // let cast2float = raw_ops::Cast::new().DstT(tensorflow::DataType::Float);
        // let img = cast2float.call(&ctx, &img)?;
        // let batch = raw_ops::expand_dims(&ctx, &img, &0)?; // add batch dim
        // let readonly_x = batch.resolve()?;
        // let img = image::load_from_memory(img_bytes.as_ref())?;

        /* create cv Mat from the image */
    //    let mut img_mat = Mat::new_rows_cols_with_default(img.height().try_into()?, img.width().try_into()?, opencv::core::CV_8UC3, Scalar::all(0.))?;
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

    }

    println!("finished");

    Ok(())
}
