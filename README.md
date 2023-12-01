# Coretempf - Output CPU Temperature Stats, the Sane Way.

This isn't just another CPU temperature printer. I created this while designing my status bar, out of frustration for the traditional approach approach of either filtering programs that don't output exactly what you want, or catting `/sys` files with long file paths you're inevitably going to assign to variables, and whose output is probably to require math. 

Doing this once is no big deal, but if you're trying to mix and match, experiment with different layouts, and you're using shell functions & aliases, grepping all sorts of programs with regular expressions that may or may not break one day, or god forbid you want median temperatures, or want to calculate your own average (package temp isn't the same)? Yeah.. That's when the "there's got to be a better way" cogs start turning.

No more `sensors | rg "^Pack[^\+]+([^ ]+)" -o -r '$1'` shenanigans.

## Okay, so what's your solution?
Sequentially evaluated formatting expressions as command line arguments.. Let me explain. Coretempf is used by providing a sequence of one of three types of arguments: a control flag, a segment, or a parameterized segment. These are then evaluated sequentially as they are being parsed, rather than first parsed and then evaluated in no particular order. Think of it like inputting code into a REPL (read, eval, print, loop), except each expression is an argument rather than a line. The output is the resulting string that each argument played a role in constructing. This makes sequence very important, especially for control flags, since unlike options, it matters whether or not you `--turn-something-on` **before** you `--output-something`, and likewise, `--turn-it-off` **after** `--outputting-something-else` 

In essence, Coretempf is a string builder. 

In the future, I might rename this project, as CPU core temperatures aren't the only thing suffering from the problem I aimed to solve. A unified way to flexibly format all sorts of performance and hardware statistics is an intriguing idea; imagine 100% (80% realistically) of the data on your status bar coming from one optimized binary, built in Rust. It would be insanely portable.

## Currently Available 

#### Segments
- Average CPU Temperature 
- Median CPU Temperature 
- Coldest Core Temperature 
- Hottest Core Temperature 
- New Line / Carriage Return 
- CPU Package Temperature 
- Unit Glyph °C °K °F  
- CPU Core Count 

#### Segments with Parameters
- Core Temperature
- Core Critical Temperature
- Core Alarm State
- Untouched String 

    
#### Control Flags
- Unit Conversion Base Unit
- Unit Conversion Target Unit
- Universally Dis/Enable Unit Glyph °C °K °F 

## Planned
- Core Frequency
- CPU Usage & Per-Core Usage
- Color Control Flags
- Built-in Presets
- Omit Cores from Output by Filter
- Control Decimal Places
- Floor, Ceil, and Round

## Usage/Documentation
TODO

## Examples

```
coretempf -t 0 1 2 3 4 5

60.00, 59.00, 58.00, 59.00, 58.00, 58.00

# Coma separation is automatic if more than one core is supplied,
# but you can -t 0 -t 1 to squash them together you like. I won't ask why.
```

```
coretempf -ug yes -s 'Avg ' -av -s ' [ Min ' -tm -s ' / Max ' -tx -s  ' ]'

Avg 58.33°C [ Min 58.00°C / Max 59.00°C ]
```

```
coretempf -s 'In ' -g -s ' the median temperature is: ' -md

In °C the median temperature is: 56.00
```

```
coretempf -s ' ------ Kelvin ------ ' -cr -tu k -t 0 1 2 -cr -t 3 4 5

 ------ Kelvin ------ 
333.15, 332.15, 331.15
332.15, 332.15, 332.15
```

```
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


