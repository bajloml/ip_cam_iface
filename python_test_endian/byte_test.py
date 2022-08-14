import os
from matplotlib import pyplot as plt
import numpy as np
import sys

def detect_overflow(value, old_value, overflow, overflow_step):
    ret = False

    if(((overflow*overflow_step)+value)<old_value):
        ret = True
    else:
        ret = False
    
    return ret

def detect_overflow2(value, old_value, overflow, overflow_step):
    num_of_oveflows = overflow

    calc_val = (overflow*overflow_step) + value

    if calc_val < old_value:

        # new_step = ((overflow*overflow_step)+old_value)
        new_step = (overflow_step - old_value) + value

        new_val = old_value + new_step

        while True:
            overflows = new_val - overflow_step
            if overflows>0:
                num_of_oveflows +=1
                new_val -=overflow_step
            else:
                break
    
    return num_of_oveflows

# create exp function without 2x overflows in one step
def create_exp(num_of_samples, words_in_sample, x_axis):
    num = 0
    power = 3
    data = []

    for i in range(int(num_of_samples * words_in_sample)):
        if ((i!=0) and i%words_in_sample==0):
            num+=1

        data.append((x_axis[num] + (i%words_in_sample * 15))**power)
    
    return data

# create exp function without 2x overflows in one step
def create_exp_2x_overflow(num_of_samples, words_in_sample):
    magic_num = 1
    num = 1
    data = []

    for i in range(int(num_of_samples * words_in_sample)):
        if(i%words_in_sample == 0) and (i!=0):
            magic_num +=0.2
            num = 1

        data.append(int((num)**magic_num))
        num+=1

    return data

# create liner function
def create_linear(num_of_samples, words_in_sample):
    num = 0
    data = []

    for i in range(int(num_of_samples * words_in_sample)):
        data.append(num*500)
        num+=1

    return data

# read binary file as chosen endian:
def bytes_to_list(bytes, endian, words_in_sample):

    WORDS_list = [[] for _ in range(words_in_sample)]
    word = 0
    read_byte_from = 0
    read_byte_to = 2
    for i in range(word_samples * words_in_sample):
        WORDS_list[word].append(int.from_bytes(bytes[read_byte_from:read_byte_to], endian))
        read_byte_from = read_byte_to
        read_byte_to +=2 
        
        word += 1

        if(word>5):
            word = 0

    return WORDS_list

# reconstruct data from list
def reconstruct_data(from_list, words_in_sample, uint16_size):
    WORDS_reconstructed = [[] for _ in range(words_in_sample)]
    word = 0
    for words in from_list:
        overflow = 0
        first_run_finished = False
        old_value = 0

        sample_counter = 0

        for value in words:
            if first_run_finished == False:
                WORDS_reconstructed[word].append(value)
                old_value = value

            if(detect_overflow(value, old_value, overflow, uint16_size) and first_run_finished):
                overflow +=1

            if first_run_finished:
                WORDS_reconstructed[word].append(value + overflow*uint16_size)
                old_value = value + overflow*uint16_size

            sample_counter += 1
            first_run_finished = True  

        word += 1

        if(word>5):
            word = 0

    return WORDS_reconstructed

print("system byteorder(endianness) is " + sys.byteorder)

file_name = os.path.realpath(os.path.join(os.getcwd(), os.path.dirname(__file__))) + "/test.log"

bytes_to_read = 12
bytes_in_word = 2
word_len = 16
words_in_sample = 6

num_of_samples = 50

uint16_size = 2**16

# x axis for ploting
x_axis = list(range(num_of_samples))

# create bytes logic
data = create_exp(num_of_samples, words_in_sample, x_axis)
# data = create_exp_2x_overflow(num_of_samples, words_in_sample)
# data = create_linear(num_of_samples, words_in_sample)

data_arr_16 = np.array(data).astype(np.uint16)
dummy_bytes = bytearray(data_arr_16)

# write dummy bytes to file
try:
    with open(file_name, "wb") as file:
        file.write(dummy_bytes)
        file.close()

except IOError:
    print("Error while opening for writing, file -> " + file_name)  

# open file and read bytes into the arrays
try:
    with open(file_name, "rb") as file:
        bytes = file.read()
        bytes_len = len(bytes)
        word_samples = int(bytes_len/bytes_to_read)
        print("bytes len = " + str(len(bytes)) + ", samples: " + str(word_samples))

        # read bytes in big endian byteorder
        WORDS_in_big = bytes_to_list(bytes, "big", words_in_sample)

        # read bytes in little endian byteorder
        WORDS_in_little = bytes_to_list(bytes, "little", words_in_sample)

except IOError:
    print("Error while opening for reading, file -> " + file_name)

# ploting in overflow, (uint16)
fig, axis = plt.subplots(2)
fig.suptitle("big and little endian")

i = 0
for words in WORDS_in_big:
    axis[0].plot(x_axis, words, label="big_val" + str(i))
    i+=1

i = 0
for words in WORDS_in_little:
    axis[1].plot(x_axis, words, label="little_val" + str(i))
    i+=1

axis[0].set_title("WORDS_in_big_endian --> " + ("correct" if sys.byteorder == "big" else "wrong representation" ) + ", because system is: " + sys.byteorder)
axis[1].set_title("WORDS_in_little_endian--> " + ("correct" if sys.byteorder == "little" else "wrong representation" ) + ", because system is: " + sys.byteorder)
axis[0].legend()
axis[1].legend()
plt.show(block=False)

# reconstruct values from uint16
WORDS_big_endian_reconstructed = reconstruct_data(WORDS_in_big, words_in_sample, uint16_size)
WORDS_little_endian_reconstructed = reconstruct_data(WORDS_in_little, words_in_sample, uint16_size)

print("WORDS_big_reconstructed:")
for i in range(len(WORDS_big_endian_reconstructed)):
    print("WORD" + str(i) +": " + str(WORDS_big_endian_reconstructed[i]))

print("WORDS_little_reconstructed:")
for i in range(len(WORDS_little_endian_reconstructed)):
    print("WORD" + str(i) +": " + str(WORDS_little_endian_reconstructed[i]))

# ploting in reconstructed values
fig, axis = plt.subplots(2)
fig.suptitle("big and little endian")

i = 0
for words in WORDS_big_endian_reconstructed:
    axis[0].plot(x_axis, words, label="big_val" + str(i))
    i+=1

i = 0
for words in WORDS_little_endian_reconstructed:
    axis[1].plot(x_axis, words, label="little_val" + str(i))
    i+=1

axis[0].set_title("WORDS_big_endian_reconstructed --> " + ("correct" if sys.byteorder == "big" else "wrong representation" ) + ", because system is: " + sys.byteorder)
axis[1].set_title("WORDS_little_endian_reconstructed --> " + ("correct" if sys.byteorder == "little" else "wrong representation" ) + ", because system is: " + sys.byteorder)
axis[0].legend()
axis[1].legend()
plt.show()

print("done")
