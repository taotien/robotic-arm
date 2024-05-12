import cv2 as cv
import numpy as np

img = cv.imread("C:\\Users\\FabienMaresquier\\Pictures\\img003.jpg")

cap = cv.VideoCapture(0)

backSub = cv.createBackgroundSubtractorMOG2()

# take first frame of the video
ret,frame = cap.read()
# setup initial location of window
x, y, w, h = 300, 230, 100, 100 # simply hardcoded the values
track_window = (x, y, w, h)
# set up the ROI for tracking
roi = frame[y:y+h, x:x+w]
hsv_roi = backSub.apply(roi)
mask = cv.inRange(hsv_roi, np.array(200.), np.array(255.))
roi_hist = cv.calcHist([hsv_roi],[0],mask,[180],[0,180])
cv.normalize(roi_hist,roi_hist,0,255,cv.NORM_MINMAX)
# Setup the termination criteria, either 10 iteration or move by at least 1 pt
term_crit = ( cv.TERM_CRITERIA_EPS | cv.TERM_CRITERIA_COUNT, 10, 1 )

while True:
    retval, frame = cap.read()
    if retval is False:
        continue

    cv.imshow("Live", frame)
    frame = backSub.apply(frame)

    hsv = frame
    dst = cv.calcBackProject([hsv], [0], roi_hist, [100, 255], 1)
    # apply camshift to get the new location
    ret, track_window = cv.CamShift(dst, track_window, term_crit)
    # Draw it on image
    pts = cv.boxPoints(ret)
    pts = np.intp(pts)
    img2 = cv.polylines(frame, [pts], True, 255, 2)
    cv.imshow('img2', img2)

    keyboard = cv.waitKey(1)
    if keyboard == 'q' or keyboard == 27:

        break