use std::{env, fs};
//use std::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;
//use std::result::Result;

use thirtyfour_sync::prelude::*;

use reqwest;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use image::*;

use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use imageproc::drawing::draw_text_mut;

use show_image::{create_window};

use rusttype::{Font, Scale};

//use tensorflow::Code;
use tensorflow::Graph;
use tensorflow::SavedModelBundle;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
//use tensorflow::Status;
use tensorflow::Tensor;
use tensorflow::DEFAULT_SERVING_SIGNATURE_DEF_KEY;

use tensorflow::eager::{self, raw_ops, ToTensorHandle};

pub struct LabelVal {
    pub name: String,
    pub color: Rgba<u8>,
}
#[show_image::main]
fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{

    // /* login auth*/
    let _username = String::from("admin");
    let _password = String::from("admin");

    /* image name */
    let _img_name = "cam_img";

    /* image dimensions to feed into the model */
    let model_image_dim = 320 as u32;
    let show_score_min = 0.30 as f32;

    let model_path = "models/ssd_mobilenet_v2_320x320_coco17_tpu-8/saved_model";
    let label_path = "models/mscoco_complete_label_map.pbtxt";

    /* option to run model through saved image or through image from memory */
    let run_through_saved_image = false;

    /* local vars */
    let mut labels: HashMap<u32, LabelVal> = HashMap::new();

    /* camera configuration and setup */
    //{
        /* start chromedriver */
        let _output = std::process::Command::new("chromedriver")
                                                    .output()
                                                    .expect("Failed to start process -> chromedrive");

        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:9515", caps)?;

        /* go to camera IP */
        driver.get("http://192.168.8.155")?;

        /* Login as user */
        let element = driver.find_element(By::XPath("//input[@value='Applet' ]"))?;
        element.click()?;

        let element = driver.find_element(By::Name("ID"))?;
        element.clear()?;
        element.send_keys(_username)?;

        let element = driver.find_element(By::Name("PassWord"))?;
        element.clear()?;
        element.send_keys(_password)?;

        let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table//tbody/tr/td/table/tbody/tr/td/p/font/input"))?;
        element.click()?;

        /* go to camera settings */
        driver.get("http://192.168.8.155/sysconfig.cgi")?;

        /* enable image on local network */
        let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/table/tbody/tr/td/input[@name='access_image' and @value=1]"))?;
        element.click()?;

        /* set image name */
        let element = driver.find_element(By::Name("img_name_drt_acs"))?;
        element.clear()?;
        element.send_keys(_img_name)?;

        /* submit changes */
        let element = driver.find_element(By::XPath("/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/input[@value='Submit']"))?;
        element.click()?;

        // Always explicitly close the browser.
        // driver.quit();
        // match driver.quit(){
        //     Ok(res) => println!("Camera configuration is done: {:?}", res),
        //     Err(error) => panic!("Problem with closing driver: {:?}", error),
        // };

        // driver.close()?;
        match driver.close(){
            Ok(res) => println!("Camera configuration is done: {:?}", res),
            Err(error) => panic!("Problem with closing driver: {:?}", error),
            // Err(error) => println!("Problem with closing driver: {:?}", error),
        };
    //}

    /* create a client */
    // let client = reqwest::Client::builder().build()?;
    let client = reqwest::blocking::Client::builder().build()?;
    let mut counter = 0 as u32;

    /* get labels and model paths */
    let current_dir = env::current_dir()?;
    let path_to_models_dir = current_dir.parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap();

    let mut path_to_model = PathBuf::from(path_to_models_dir);
    path_to_model.push(model_path);

    let mut path_to_labels = PathBuf::from(path_to_models_dir);
    path_to_labels.push(label_path);

    /* get labels str */
    let labels_str = fs::read_to_string(path_to_labels.display().to_string());
    match labels_str{
        Ok(_) => println!("Labels ({:?}) found", path_to_labels.as_os_str()),
        Err(error) => panic!("Problem with opening {:?}: {:?}", path_to_labels.as_os_str(), error),
    };

    /* get label data into a hash map */
    let mut labels_str = labels_str.unwrap();
    let label_field_name = "name";
    let label_field_id = "id";
    let label_field_display_name = "display_name";
    print!("Loading labels... ");
    while labels_str.len() > 1 {

        /* get label item string boundaries */
        let label_description_begin = labels_str.find('{').unwrap();
        let label_description_end = labels_str.find('}').unwrap() + 1;

        /* get label item data */
        let item = &labels_str[label_description_begin..label_description_end].to_string();

        /* get label name to create color */
        let item_begin = item.find(label_field_name).unwrap();
        let item_end = item_begin + (&item[item_begin..]).to_string().find('\n').unwrap() + 1;
        let name = item[item_begin + label_field_name.len() + 3.. item_end - 2].trim().to_string();

        let mut value: u64 = 0;
        for byte in name.as_bytes().iter(){
            value += *byte as u64;
        }
        /* set the seed and get color number */
        let mut rand_gen = StdRng::seed_from_u64(value);
        let red = rand_gen.gen_range(0..255) as u8;
        let green = rand_gen.gen_range(0..255) as u8;
        let blue = rand_gen.gen_range(0..255) as u8;

        /* get label id */
        let item_begin = item.find(label_field_id).unwrap();
        let item_end = item_begin + (&item[item_begin..]).to_string().find('\n').unwrap() + 1;
        let id = (&item[item_begin + label_field_id.len() + 2..item_end].trim()).parse::<u32>().unwrap();

        /* get label display name */
        let item_begin = item.find(label_field_display_name).unwrap();
        let item_end = item_begin + (&item[item_begin..]).to_string().find('\n').unwrap() + 1;
        let display_name = item[item_begin + label_field_display_name.len() + 3.. item_end - 2].trim().to_string();

        /* add to ids as keys and LabelVal as value in hash map */
        labels.entry(id).or_insert(LabelVal{name:display_name, color: image::Rgba([red, green, blue, 0])});

        /* move to next item */
        labels_str = labels_str[label_description_end..].to_string();
    }
    println!("loaded!");

    /* Load the model.*/
    print!("Loading a model: {}", path_to_model.to_str().unwrap());
    let model_loading_time = std::time::Instant::now();
    let mut graph = Graph::new();
    let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, path_to_model)?;
    let session = &bundle.session;
    println!(" loaded in {:?}", model_loading_time.elapsed());

    // Create an eager execution context
    let opts = eager::ContextOptions::new();
    let ctx = eager::Context::new(opts)?;

    // get in/out operations
    let signature = bundle.meta_graph_def().get_signature(DEFAULT_SERVING_SIGNATURE_DEF_KEY)?;

    let input_info = signature.get_input("input_tensor")?;

    let output_info_boxes = signature.get_output("detection_boxes")?;
    let output_info_classes = signature.get_output("detection_classes")?;
    let output_info_scores = signature.get_output("detection_scores")?;

    // Create a window with default options and display the image.
    let window = create_window("image with detections", Default::default())?;

    loop{
        /* measure time */
        let detection_time = std::time::Instant::now(); 
                                            
        /* get image from url */
        let img_bytes = client.get(format!("http://192.168.8.155/jpg/{}{}", _img_name, ".jpg")).send().unwrap().bytes().unwrap();

        let img = image::load_from_memory_with_format(img_bytes.as_ref(), image::ImageFormat::Jpeg)?;
        let img_resized = image::imageops::resize(&img, model_image_dim, model_image_dim, image::imageops::FilterType::Nearest);

        /* create a tensor to load into model */
        let mut input: Tensor<u8> = Tensor::new(&[1, model_image_dim as u64, model_image_dim as u64, 3]);

        /* fill input tensor with image from memory or from image from file */
        if !run_through_saved_image {

            let img_pixels = img_resized.pixels();
            let mut vec_flattened: Vec<u8> = Vec::new();
            for rgb in img_pixels{
                vec_flattened.push(rgb[2] as u8);
                vec_flattened.push(rgb[1] as u8);
                vec_flattened.push(rgb[0] as u8);
            }

            //The `input` tensor expects BGR pixel data.
            input = input.with_values(&vec_flattened).unwrap();
        }
        else{       
            /* save image to run on model */
            let save_image_path = "saved_image.jpg";
            img_resized.save(save_image_path)?;

            // Create input tensor from previously saved image and load it in a batch of shape 1,320,320,3.
            let fname = save_image_path.to_handle(&ctx)?;
            let buf = raw_ops::read_file(&ctx, &fname)?;
            let img_tensor = raw_ops::decode_image(&ctx, &buf)?;
            // let cast2 = raw_ops::Cast::new().DstT(tensorflow::DataType::Float);
            // let img_tensor = cast2.call(&ctx, &img_tensor)?;
            let batch = raw_ops::expand_dims(&ctx, &img_tensor, &0)?; // add batch dim at position 0 to have 1,320,320,3
            let readonly_x = batch.resolve()?;

            // The current eager API implementation requires unsafe block to feed the tensor into a graph.
            input = unsafe { readonly_x.into_tensor() };
        }

        /* create arguments to feed as inputs and to request outputs from model */
        let mut args = SessionRunArgs::new();
        
        args.add_feed(&graph.operation_by_name_required(&input_info.name().name)?, 0, &input);

        let token_output_boxes = args.request_fetch(&graph.operation_by_name_required(&output_info_boxes.name().name)?, 1);
        let token_output_classes = args.request_fetch(&graph.operation_by_name_required(&output_info_classes.name().name)?, 2);
        let token_output_scores = args.request_fetch(&graph.operation_by_name_required(&output_info_scores.name().name)?, 4);

        /* Run the graph. */
        session.run(&mut args)?;

        /* Check the output. */
        let output_boxes: Tensor<f32> = args.fetch(token_output_boxes)?;
        let output_classes: Tensor<f32> = args.fetch(token_output_classes)?;
        let output_scores: Tensor<f32> = args.fetch(token_output_scores)?;

        /* draw on copy image */
        let mut img_out = img_resized.clone();

        /* draw rectangles, write text on output image from model outputs (detections) */
        for (bbox, score, class) in itertools::izip!(output_boxes.chunks_exact(4), output_scores.iter(), output_classes.iter()){
            /* y1 = bbox0, y2 = bbox2, x1 = bbox1, x2 = bbox3 */
            if *score > show_score_min{

                //Create a `Rect` from the bounding box.
                let pos_x = (bbox[1]* model_image_dim as f32) as i32;
                let pos_y = (bbox[0] * model_image_dim as f32) as i32;
                let width = ((bbox[3] - bbox[1]) * model_image_dim as f32) as u32;
                let height = ((bbox[2] - bbox[0]) * model_image_dim as f32) as u32;
                let rect = Rect::at(pos_x, pos_y).of_size(width as u32, height as u32);

                // Draw a green line around the bounding box
                draw_hollow_rect_mut(&mut img_out, rect, labels.get(&(*class as u32)).unwrap().color);

                // Write label class and score text on bounding box
                let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
                let font = Font::try_from_vec(font).unwrap();

                draw_text_mut(&mut img_out,
                              labels.get(&(*class as u32)).unwrap().color, 
                              pos_x, 
                              pos_y, 
                              Scale {x: 15 as f32, y: 15 as f32},
                              &font, 
                              format!("{} {}", labels.get(&(*class as u32)).unwrap().name.as_str(), (*score).to_string().as_str()).as_str()
                            );
            }
        }

        // Save the image
        img_out.save("detections.jpeg")?;

        let img = show_image::Image::from(img_out);
        let img_out_view = img.as_image_view().unwrap();

        // Create a window with default options and display the image.
        window.set_image("image-001", img_out_view)?;

        counter += 1;
        println!("counter = {}, elapsed time = {:?}", counter, detection_time.elapsed());
    }

    Ok(())
}
