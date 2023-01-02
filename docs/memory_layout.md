# Sprite Memory Layout

In this implementation, sprites are always initialized as the following:

```
 Addr | 00 01 02 03 04
 ---------------------
 000  | F0 90 90 90 F0  # Sprite data for '0'
 005  | 20 60 20 20 70  # Sprite data for '1'
 00A  | F0 10 F0 80 F0  # Sprite data for '2'
 00F  | F0 10 F0 10 F0  # Sprite data for '3'
 014  | 90 90 F0 10 10  # Sprite data for '4'
 019  | F0 80 F0 10 F0  # Sprite data for '5'
 01E  | F0 80 F0 90 F0  # Sprite data for '6'
 023  | F0 10 20 40 40  # Sprite data for '7'
 028  | F0 90 F0 90 F0  # Sprite data for '8'
 02D  | F0 90 F0 10 F0  # Sprite data for '9'
 032  | F0 90 F0 90 90  # Sprite data for 'A'
 037  | E0 90 E0 90 E0  # Sprite data for 'B'
 03C  | F0 80 80 80 F0  # Sprite data for 'C'
 041  | E0 90 90 90 E0  # Sprite data for 'D'
 046  | F0 80 F0 80 F0  # Sprite data for 'E'
 04B  | F0 80 F0 80 80  # Sprite data for 'F'
```
