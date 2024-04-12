// #include "lib/Braccio/Braccio.h"
// #include "lib/Servo/Servo.h"

#include <Braccio.h>
#include <Servo.h>

Servo base;
Servo shoulder;
Servo elbow;
Servo wrist_ver;
Servo wrist_rot;
Servo gripper;

enum Axis {
  Base,
  Shoulder,
  Elbow,
  Wrist,
  Rotation,
};

struct Position {};

void setup() {
  Serial.begin(115200);
  while (!Serial) {
    ;
  }

  Serial.println("hello");
  Serial.setTimeout(60000);

  // Braccio.begin()
}

void loop() {
  // Braccio.ServoMovement(20, 53, 90, 90, 90, 90, 73);

  /*
Step Delay: a milliseconds delay between the movement of each servo.  Allowed
values from 10 to 30 msec. M1=base degrees. Allowed values from 0 to 180 degrees
M2=shoulder degrees. Allowed values from 15 to 165 degrees
M3=elbow degrees. Allowed values from 0 to 180 degrees
M4=wrist vertical degrees. Allowed values from 0 to 180 degrees
M5=wrist rotation degrees. Allowed values from 0 to 180 degrees
M6=gripper degrees. Allowed values from 10 to 73 degrees. 10: the toungue is
open, 73: the gripper is closed.
*/
  M1 = atan(y_target / x_target) * 180.0 / PI;

  if (x_target < 0) {
    M1 = 180 - M1;
  }

  String input = String();
  if (Serial.available()) {
    input = Serial.readStringUntil('\n');
    unsigned int length = input.length();
    // Serial.print(input);
    switch (input[0]) {
    case 'b':
      // String out = "degrees: " + input.substring(1).toInt();
      Serial.println(input.substring(1).toInt());
      break;
    case 's':
      break;
    case 'e':
      break;
    case 'w':
      break;
    case 'r':
      break;
    }
  }
}
