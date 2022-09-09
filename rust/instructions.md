## Camera interface:
- camera image is taken from the local url
- image is fed model
- output image(with detections) is shown and saved


## note:
- tensorflow shared libs (.so) are installed manually in ```/lib/x86_64-linux-gnu```
- check if correct ```tensorflow .so``` files are installed on system
- if there are no ```tensorflow .so``` files, download and install those from [C API](https://www.tensorflow.org/install/lang_c)
- python script is used to test various model to check which one is the best fit
- rust is used to make detections faster


## before run make sure chromedriver is running
