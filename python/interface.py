#!/usr/bin/env python

from time import sleep
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from sqlalchemy import table
from webdriver_manager.chrome import ChromeDriverManager
import cv2
import numpy
import urllib.request

from PIL import Image
import requests
from io import BytesIO

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

counter = 0

# open image url
while True:
    req = urllib.request.urlopen("http://192.168.8.155/jpg/" + img_name +".jpg")
    arr = numpy.asarray(bytearray(req.read()), dtype=numpy.uint8)
    opencvImage = cv2.imdecode(arr, -1)

    cv2.imshow(img_name, opencvImage)
    cv2.waitKey(10)

    counter += 1
    print("counter = " + str(counter))

