### tested on few models from tensorflow repo
Thanks to:
@misc{tensorflowmodelgarden2020,
  author = {Hongkun Yu, Chen Chen, Xianzhi Du, Yeqing Li, Abdullah Rashwan, Le Hou, Pengchong Jin, Fan Yang,
            Frederick Liu, Jaeyoun Kim, and Jing Li},
  title = {{TensorFlow Model Garden}},
  howpublished = {\url{https://github.com/tensorflow/models}},
  year = {2020}
}

### testing:
- download model to use ([tf2 detection model ZOO](https://github.com/tensorflow/models/blob/master/research/object_detection/g3doc/tf2_detection_zoo.md) by right click on model to download and then copy link and use that link in another tab)
- download label data to use ([label data](https://github.com/tensorflow/models/blob/master/research/object_detection/data/mscoco_complete_label_map.pbtxt))
- extract the downloaded model ```mscoco_complete_label_map.pbtxt``` to ```models``` directory  (in root of the repository)
- copy ```mscoco_complete_label_map.pbtxt``` to ```models``` directory  (in root of the repository)
- run the script ```interface_tf.py```

