import os
from matplotlib import pyplot as plt
import numpy as np

file_name = os.path.realpath(os.path.join(os.getcwd(), os.path.dirname(__file__))) + "/test.log"

bytes_to_read = 12
bytes_in_word = 2
word_len = 16
words_in_sample = 6

num_of_samples = 50

# create bytes 
data = []
magic_num = 1
num = 1
# for i in range(int(num_of_samples * bytes_to_read / bytes_in_word)):
#     if(i%words_in_sample == 0) and (i!=0):
#         magic_num +=0.2
#         num = 1

#     data.append(int((num)**magic_num))
#     num+=1

for i in range(int(num_of_samples * bytes_to_read / bytes_in_word)):
    data.append(num+num)
    num+=1

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
        x_axis = []

        bytes = file.read()
        bytes_len = len(bytes)
        word_samples = int(bytes_len/bytes_to_read)
        print("bytes len = " + str(len(bytes)) + ", samples: " + str(word_samples))

        # axis x in plot
        for i in range(word_samples):
            x_axis.append(i)

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


# test with no uint16
WORDS_in_big_test = [[] for _ in range(words_in_sample)]
WORDS_in_little_test = [[] for _ in range(words_in_sample)]

word = 0
for words in WORDS_in_big:
    overflow = False
    overflow_passed = False
    first_run_finished = False
    old_value = 0

    sample_counter = 0

    for value in words:
        if first_run_finished == False:
            WORDS_in_big_test[word].append(value)
            old_value = value

        if((value < old_value) and first_run_finished):
            overflow = True

        if overflow:
            WORDS_in_big_test[word].append(value + 65535)
            old_value = value + 65535

        elif(first_run_finished):
            if overflow:
                WORDS_in_big_test[word].append(value + 65535)
                old_value = value + 65535
            else:
                WORDS_in_big_test[word].append(value)
                old_value = value

        sample_counter += 1
        first_run_finished = True  

    word += 1

    if(word>5):
        word = 0

word = 0
for words in WORDS_in_little:
    overflow = False
    first_run_finished = False
    old_value = 0

    sample_counter = 0

    for value in words:
        if first_run_finished == False:
            WORDS_in_little_test[word].append(value)
            old_value = value

        if((value < old_value) and first_run_finished):
            overflow = True

        if overflow:
            WORDS_in_little_test[word].append(value + 65535)
            old_value = value + 65535

        elif(first_run_finished):
            if overflow:
                WORDS_in_little_test[word].append(value + 65535)
                old_value = value + 65535
            else:
                WORDS_in_little_test[word].append(value)
                old_value = value

        sample_counter += 1
        first_run_finished = True  

    word += 1

    if(word>5):
        word = 0

# ploting in overflow, (uint16)
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
