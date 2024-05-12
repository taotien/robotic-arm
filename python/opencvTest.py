import cv2 as cv
import numpy as np

img = cv.imread("C:\\Users\\FabienMaresquier\\Pictures\\img003.jpg")

cap = cv.VideoCapture(0)

frame = np.array([], dtype=int)

backSub = cv.createBackgroundSubtractorMOG2()

while True:
    retval, frame = cap.read()
    if frame is None:
        continue

    fgMask = backSub.apply(frame)

    cv.imshow("Live", frame)
    cv.imshow("Mask", fgMask)

    keyboard = cv.waitKey(1)
    if keyboard == 'q' or keyboard == 27:
        break