#!.venv/bin/python3
import subprocess
import sys
import cv2
import os
import numpy as np

def crop_to_square(frame):
    (h, w) = frame.shape[:2]
    if w > h:
        # Crop to square
        frame = frame[:, (w - h) // 2:(w + h) // 2]
    elif h > w:
        frame = frame[(h - w) // 2:(h + w) // 2, :]
    return frame

if __name__ == "__main__":
    name = sys.argv[1]

    input_file = sys.argv[1]
    name = '.'.join(os.path.basename(input_file).split('.')[:-1])

    cap = cv2.VideoCapture(input_file)

    if not cap.isOpened(): 
        print(f"Could not open {input_file}")
        exit(1)

    # Get video metadata
    num_frames = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
    input_width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
    input_height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))

    cap.release()

    # Memory map a file to store the raw bytes
    buffer = np.memmap(f"src/animations/{name}.raw", mode="w+", shape=(num_frames, 8, 8, 3), dtype=np.uint8)
    print(f"Converting {num_frames} frames from {input_file}...")


    # Use ffmpeg to sample the video at 24 fps and convert it to raw bytes
    command = [
        'ffmpeg',
        '-i', input_file,
        '-r', '24',
        '-pix_fmt', 'rgb24',
        '-vcodec', 'rawvideo',
        '-an', '-sn',   # disable audio processing
        '-f', 'image2pipe', '-'
    ]

    pipe = subprocess.Popen(command, stdout=subprocess.PIPE, bufsize=10**8)
    if pipe.stdout is None:
        print(f"Could not open {input_file}")
        exit(1)

    # Read the raw bytes from the pipe and store them in the buffer
    i = 0
    while True:
        raw_image = pipe.stdout.read(input_width * input_height * 3)
        if len(raw_image) != input_width * input_height * 3:
            break

        frame = np.frombuffer(raw_image, dtype=np.uint8).reshape((input_height, input_width, 3))
        frame = crop_to_square(frame)
        frame = cv2.resize(frame, (8, 8), interpolation=cv2.INTER_AREA)
        buffer[i] = frame
        i += 1

    print(f"Converted {i} frames to raw.")
    

    
