# Coretempf - The Sane Way to Output Core Temps

Outputting core temperatures, in a user-specified format, akin to `printf`. No more `sensors | rg "^Pack[^\+]+([^ ]+)" -o -r '$1'` shenanigans.

This isn't just another core temperature printer. I created this while designing my status bar, out of frustration for the traditional approach approach of either filtering programs that don't output exactly what you want, or catting `/sys` files with long file paths whose output is probably to require math to get it human readable. Doing this once is no big deal, but if you're trying to mix and match, experiment with different layouts, and you're using shell functions & aliases, grepping all sorts of programs with regular expressions that may or may not break one day, or god forbid you want median temperatures, or want to calculate your own average (package temp isn't the same)? Yeah.. that's when the "there's got to be a better way" cogs start turning.

## Sequentially Evaluated Expression
What does coretempf do differently? A lot, but it boils down to command line arguments being a sequentially interpreted formatting expression, rather than just arguments. What do I mean by this? Let me demonstrate.

```bash
>> coretempf -t 0 1 2 3 4 5
```
```
60.00, 59.00, 58.00, 59.00, 58.00, 58.00
```

```bash
>> coretempf -ug yes -s 'Avg ' -av -s ' [ Min ' -tm -s ' / Max ' -tx -s  ' ]'
```
```
Avg 58.33°C [ Min 58.00°C / Max 59.00°C ]

```

```bash
coretempf -ug yes \
  -s '---------------------------------'    -cr \
  -s ' '  -av   -s ' /  AVG | MED  \\ ' -md -cr \
  -s '----------------|----------------'    -cr \
  -s ' '  -tm   -s ' \\  MIN | MAX  / ' -tm -cr \
  -s '----------------|----------------'    -cr \
  -s 'Core 1: ' -t 0 -s ' | Core 2: ' -t 1  -cr \
  -s 'Core 3: ' -t 2 -s ' | Core 4: ' -t 3  -cr \
  -s 'Core 5: ' -t 4 -s ' | Core 6: ' -t 5  -cr \
  -s '---------------------------------'             
```

```
---------------------------------
 58.17°C /  AVG | MED  \ 59.00°C
----------------|----------------
 57.00°C \  MIN | MAX  / 57.00°C
----------------|----------------
Core 1: 59.00°C | Core 2: 58.00°C
Core 3: 57.00°C | Core 4: 59.00°C
Core 5: 59.00°C | Core 6: 57.00°C
---------------------------------
```

```bash
>> coretempf -s ' ------ Kelvin ------ ' -cr -tu k -t 0 1 2 -cr -t 3 4 5
```

```
 ------ Kelvin ------ 
333.15, 332.15, 331.15
332.15, 332.15, 332.15
```


