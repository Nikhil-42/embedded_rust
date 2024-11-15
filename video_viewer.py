import cv2
import sys
import os
import numpy as np

if __name__ == "__main__":
    num_frames = os.path.getsize(f"src/animations/{sys.argv[1]}.raw") // 8 // 8 // 3
    buffer = np.memmap(f"src/animations/{sys.argv[1]}.raw", mode="r", shape=(num_frames, 8, 8, 3), dtype=np.uint8)
    for frame in buffer:
        cv2.imshow("image", frame[:, :, ::-1] / 255.0)
        cv2.waitKey(42)  # 24 fps

    
