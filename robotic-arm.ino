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
	while (!Serial) {;}

	Serial.println("hello");
	Serial.setTimeout(60000);
	
	// Braccio.begin()
}

void loop() {
	// Braccio.ServoMovement(20, 90, 90, 90, 90, 90, 73);

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
