import os
from matplotlib import pyplot as plt
import numpy as np

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
    power = 2
    data = []

    for i in range(int(num_of_samples * words_in_sample)):
        if ((i!=0) and i%words_in_sample==0):
            num+=1

        data.append((x_axis[num] + (i%words_in_sample * 10))**power)
    
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
        data.append(num*5)
        num+=1

    return data


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
print("dummy bytes = " + str(dummy_bytes))

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
        WORDS_in_big = [[] for _ in range(words_in_sample)]
        WORDS_in_little = [[] for _ in range(words_in_sample)]

        bytes = file.read()
        bytes_len = len(bytes)
        word_samples = int(bytes_len/bytes_to_read)
        print("bytes len = " + str(len(bytes)) + ", samples: " + str(word_samples))

        # endian big
        endian = "big"
        word = 0
        read_byte_from = 0
        read_byte_to = 2
        for i in range(word_samples * words_in_sample):
            WORDS_in_big[word].append(int.from_bytes(bytes[read_byte_from:read_byte_to], endian))
            read_byte_from = read_byte_to
            read_byte_to +=2 
            
            word += 1

            if(word>5):
                word = 0

        print("WORDS_in_big:")
        i = 0
        for words in WORDS_in_big:
            print("WORD" + str(i) +": " + str(words))
            i+=1

        # endian little
        endian = "little"
        word = 0
        read_byte_from = 0
        read_byte_to = 2
        for i in range(word_samples * words_in_sample):
            WORDS_in_little[word].append(int.from_bytes(bytes[read_byte_from:read_byte_to], endian))
            read_byte_from = read_byte_to
            read_byte_to +=2 
            
            word += 1

            if(word>5):
                word = 0

        print("WORDS_in_little:")
        i = 0
        for words in WORDS_in_little:
            print("WORD" + str(i) +": " + str(words))
            i+=1

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

axis[0].set_title("WORDS_in_big_endian")
axis[1].set_title("WORDS_in_little_endian")
axis[0].legend()
axis[1].legend()
plt.show(block=False)

# reconstruct values from uint16
WORDS_in_big_test = [[] for _ in range(words_in_sample)]
WORDS_in_little_test = [[] for _ in range(words_in_sample)]

word = 0
for words in WORDS_in_big:
    overflow = 0
    first_run_finished = False
    old_value = 0

    sample_counter = 0

    for value in words:
        if first_run_finished == False:
            WORDS_in_big_test[word].append(value)
            old_value = value

        if(detect_overflow(value, old_value, overflow, uint16_size) and first_run_finished):
            overflow +=1
        # overflow = detect_overflow2(value, old_value, overflow, uint16_size)

        if first_run_finished:
            WORDS_in_big_test[word].append(value + overflow*uint16_size)
            old_value = value + overflow*uint16_size

        sample_counter += 1
        first_run_finished = True  

    word += 1

    if(word>5):
        word = 0

word = 0
for words in WORDS_in_little:
    overflow = 0
    first_run_finished = False
    old_value = 0

    sample_counter = 0

    for value in words:
        if first_run_finished == False:
            WORDS_in_little_test[word].append(value)
            old_value = value

        if(detect_overflow(value, old_value, overflow, uint16_size) and first_run_finished):
            overflow +=1
        # overflow = detect_overflow2(value, old_value, overflow, uint16_size)

        if first_run_finished:
            WORDS_in_little_test[word].append(value + overflow*uint16_size)
            old_value = value + overflow*uint16_size

        sample_counter += 1
        first_run_finished = True  

    word += 1

    if(word>5):
        word = 0

# ploting in reconstructed values
fig, axis = plt.subplots(2)
fig.suptitle("big and little endian")

i = 0
for words in WORDS_in_big_test:
    axis[0].plot(x_axis, words, label="big_val" + str(i))
    i+=1

i = 0
for words in WORDS_in_little_test:
    axis[1].plot(x_axis, words, label="little_val" + str(i))
    i+=1

axis[0].set_title("WORDS_in_big_endian")
axis[1].set_title("WORDS_in_little_endian")
axis[0].legend()
axis[1].legend()
plt.show()

print("done")
