#!/usr/bin/env python

import imp
from time import sleep, time
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from sqlalchemy import table
from webdriver_manager.chrome import ChromeDriverManager
import cv2
import urllib.request

import os

import tkinter
from PIL import Image
import matplotlib.pyplot as plt
import numpy as np
os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2' # INFO and WARNING messages are not logged.
import tensorflow as tf2
from object_detection.utils import label_map_util
from object_detection.utils import visualization_utils as viz_utils
print(tf2.version.VERSION)

#################################################################################################################

# choose background gui for matplotlib
import matplotlib
print("Before, Backend used by matplotlib is: ", matplotlib.get_backend())
matplotlib.use('TKAgg', force=True)
print("After, Backend used by matplotlib is: ", matplotlib.get_backend())

# print matplotlib backends
import matplotlib as m; print('I: {}\nN: {}'.format(m.rcsetup.interactive_bk,m.rcsetup.non_interactive_bk));

# Root directory of the project
ROOT_DIR = os.path.abspath("./")
# model_path = ROOT_DIR + "/python/interface_tf/centernet_resnet50_v2_512x512_kpts_coco17_tpu-8/saved_model"
# model_path = ROOT_DIR + "/python/interface_tf/centernet_resnet50_v2_512x512_coco17_tpu-8/saved_model"

model_path = ROOT_DIR + "/python/interface_tf/ssd_mobilenet_v2_320x320_coco17_tpu-8/saved_model"    # fastest evaluation
# model_path = ROOT_DIR + "/python/interface_tf/centernet_hg104_512x512_kpts_coco17_tpu-32/saved_model"     # takes forever to load and evaluate image

labels_path = ROOT_DIR + "/python/interface_tf/mscoco_complete_label_map.pbtxt"

#################################################################################################################

# credentials
username = "admin"
password = "admin"

# image_name
img_name = "cam_img"

# view program (Applet or ActiveX)
viewing_program = "Applet"

# initialize the Chrome driver
driver = webdriver.Chrome("chromedriver")
# driver = webdriver.Chrome(ChromeDriverManager().install())

# open login page
driver.get("http://192.168.8.155")

# set viewing program
driver.find_element(By.XPATH, "//input[@value='"+ viewing_program +"' ]").click()

# find username and password fields
username_box = driver.find_element(By.NAME, "ID")
username_box.clear()
username_box.send_keys(username)

password_box = driver.find_element(By.NAME, "PassWord")
password_box.clear()
password_box.send_keys(password)

# click login button
driver.find_element(By.XPATH, "/html/body/form/table/tbody/tr/td/table//tbody/tr/td/table/tbody/tr/td/p/font/input").click()

#open settings configuration page
driver.get("http://192.168.8.155/sysconfig.cgi")

# enable image on local network
driver.find_element(By.XPATH, "/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/table/tbody/tr/td/input[@name='access_image' and @value=1]").click()

# set image name
image_name = driver.find_element(By.NAME, "img_name_drt_acs")
image_name.clear()
image_name.send_keys(img_name)

# submit changes
driver.find_element(By.XPATH, "/html/body/form/table/tbody/tr/td/table/tbody/tr/td/div/input[@value='Submit']").click()

# close driver 
driver.close()

# model config:
print('Loading model...')
start_time = time()
model = tf2.saved_model.load(model_path)
#model = tf2.keras.models.load_model(model_path)
end_time = time()
print('Model load done, loading lasted {} seconds'.format(end_time - start_time))

# this works with changing the line 137 in file "label_map_util.py" from "tf.gfile.GFile" to "tf.io.gfile.GFile"
category_index = label_map_util.create_category_index_from_labelmap(labels_path,
                                                                    use_display_name=True)

counter = 0

# open image url
while True:

    start = time()

    req = urllib.request.urlopen("http://192.168.8.155/jpg/" + img_name +".jpg")
    arr = np.asarray(bytearray(req.read()), dtype=np.uint8)

    # start_time = time()
    # resize image and convert it to numpy array
    # image = Image.open(ROOT_DIR + "/python/interface_tf/cam_img.jpg")
    # # image = Image.open(ROOT_DIR + "/python/interface_tf/dogs2.jpg")
    # image_resized = image.resize((512, 512))
    # image_resized.show()
    # np_image = np.array(image_resized)

    # opencvImage_resized = cv2.resize(src=cv2.imdecode(arr, -1), dsize=(512, 512), interpolation = cv2.INTER_AREA)
    opencvImage_resized = cv2.resize(src=cv2.imdecode(arr, -1), dsize=(320, 320), interpolation = cv2.INTER_AREA)

    np_image = np.asarray(opencvImage_resized)

    # convert numpy array to tensor
    input_tensor = tf2.convert_to_tensor(np_image)

    # The model expects a batch of images, so add an axis with `tf.newaxis` at the first position to get tensor of shape (1,512,512,3)
    input_tensor = input_tensor[tf2.newaxis, ...]

    # try to detect on image
    detections = model(input_tensor)

    # All outputs are batches tensors.
    # Convert to numpy arrays, and take index [0] to remove the batch dimension.
    # We're only interested in the first num_detections.
    num_detections = int(detections.pop('num_detections'))
    detections = {key: value[0, :num_detections].numpy()
                    for key, value in detections.items()}
    detections['num_detections'] = num_detections

    # detection_classes should be ints.
    detections['detection_classes'] = detections['detection_classes'].astype(np.int64)

    # end_time = time()
    # print("image processing lasted {}", end_time - start_time)

    image_np_with_detections = np_image.copy()

    viz_utils.visualize_boxes_and_labels_on_image_array(
                image_np_with_detections,
                detections['detection_boxes'],
                detections['detection_classes'],
                detections['detection_scores'],
                category_index,
                use_normalized_coordinates=True,
                max_boxes_to_draw=20,
                min_score_thresh=.30,
                agnostic_mode=False)

    # plt.figure()
    # plt.imshow(image_np_with_detections)
    # plt.show(block = False)

    cv2.imshow(img_name, image_np_with_detections)
    cv2.waitKey(1)


    #opencvImage = cv2.imdecode(arr, -1)

    #cv2.imshow(img_name, opencvImage)
    #cv2.waitKey(1)


    counter += 1

    end = time()
    print("counter = " + str(counter) + " elapsed time = " + str(end - start))

