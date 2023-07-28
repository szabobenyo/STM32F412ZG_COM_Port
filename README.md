# STM32F412ZG_COM_Port
Hello my name is Bence and I'm a trainee at BALLUFF Electronics Ltd. <br />
My current task is to familiarize myself with this microcontroller, and implement a USB interface on it.  <br />

### What the code currently do ?
- It turns ON the green LED when the device is ready.
- If you send 'L' to the USB interface, the blue LED lights up/out, depend on the current state of the LED.
- If you press the 'User button', the red LED became active. If you released the button, the LED lights off. <br />
Meanwhile the blue LED is ON, and you press this 'User button', the mcu send a message: "Someone pressed the button". <br />
This makes the communication to a duplex connectivity. <br />

The programmend keys are [1,2,3,L,B].
- (1,2,3): You recieve a message like: "You sent 'x'. Noice!" (Where x is the sended number).
- (L): It controlls the blue LED.
- (B): Retrieve 2 types of message: 1: "The button is under pressure" - 2: "The button is in released state".
