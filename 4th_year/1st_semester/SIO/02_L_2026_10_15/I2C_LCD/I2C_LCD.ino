#include <Wire.h>
#include <LiquidCrystal_I2C.h>

LiquidCrystal_I2C lcd(0x27, 16, 2);

String staticText  = "Hello world!";
String dynamicText = "F the society and this world and everything";

enum ScrollDirection {
  LEFT,
  RIGHT
};

void setup() {
  lcd.init();
  lcd.backlight();

  lcd.setCursor(0, 0);
  lcd.print(staticText);
}

void scrollText(String text, ScrollDirection direction) {
  int len   = text.length();
  int start = (direction == RIGHT) ? -len : 16;
  int end   = (direction == RIGHT) ? 16 : -len;
  int step  = (direction == RIGHT) ? 1 : -1;

  for (int pos = start; pos != end + step; pos += step) {
    for (int col = 0; col < 16; col++) {
      int index = col - pos;

      lcd.setCursor(col, 1);
      lcd.print((index >= 0 && index < len) ? text[index] : ' ');
    }

    delay(150);
  }
}

void loop() {
  scrollText(dynamicText, LEFT);
  delay(500);

  scrollText(dynamicText, RIGHT);
  delay(500);
}