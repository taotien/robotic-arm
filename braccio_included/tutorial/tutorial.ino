#include <Braccio.h>
#include <Servo.h>

Servo base;
Servo shoulder;
Servo elbow; 
Servo wrist_ver;
Servo wrist_rot;
Servo gripper;

void setup() {
  Braccio.begin()
}

void loop() {
                     //(Setup delay   M1, M2, M3, M4, M5, M6);
  Braccio.ServoMovement(20,           90, 90, 90, 90, 90, 73);
}