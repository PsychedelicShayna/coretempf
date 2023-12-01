pub const USAGE: &str = "
Usage: pcoretempf [--help | -h] [--SEGMENT | -S [ARGS]]...
where SEGMENT, ARGS... 

The output is determined entirely by what segments are given,
and in what order they are given.

Consider a segment to be a %s in a printf format string, where
%s is replaced with the appropriate value. 

Some segments take arguments, which is everything following that
segment, until the next segment.

The following segments, -ug, -s, -cr, -t, when laid out like this,
and given these parameters:

        -ug yes \\
        -s '---------------------------------' -cr \\
        -s 'Core 1: ' -t 0 -s ' | Core 2: ' -t 1 -s -cr \\
        -s 'Core 3: ' -t 2 -s ' | Core 4: ' -t 3 -s -cr \\
        -s 'Core 5: ' -t 4 -s ' | Core 6: ' -t 5 -s -cr \\
        -s '---------------------------------' -cr

Produces this output: 
        ---------------------------------
        Core 1: 53.32°C | Core 2: 53.32°C
        Core 3: 53.32°C | Core 4: 53.32°C
        Core 5: 53.32°C | Core 6: 53.32°C
        ---------------------------------

Standalone Segments (no arguments):

    --glyph   (-g)
        Print a temperature glyph for the current base unit.
        The default base unit is celcius, so the default glyph 
        is °C, but could also be: °F  °K 


    --avg (-av)
        The current average core temperature.


    --median (-md)
        The current median core temperature.


    --min (-m)
        The lowest current core temperature.


    --max (-mx)
        The highest current core temperature.


    --package (-pk)
        The current package temperature.


    --newline (-nl | -cr | -\\n)
        Prints a newline character.


    --core-count (-cc)
        Prints the total number of cores.


Parameterized Segments (one or more arguments):

    --base-unit (-bu) UNIT (default: celcius)
        Sets the base unit to UNIT, which can be:

        For Celcius: °C, c, C, celcius, Celcius
        For Farenheit: °F, f, F, fahrenheit, Fahrenheit
        For Kelvin: °K, k, K, kelvin, Kelvin


    --target-unit (-tu) UNIT (default: none)
        Sets the target unit to UNIT, which can be:

        For Celcius: °C, c, C, celcius, Celcius
        For Farenheit: °F, f, F, fahrenheit, Fahrenheit
        For Kelvin: °K, k, K, kelvin, Kelvin

        If no target unit is set, no conversion is performed.

        If a target unit is set, then any temperature values will
        be converted from the base unit to the target unit.


    --use-glyph (-ug) BOOL (default: false)
        If BOOL is true, then a temperature glyph will be printed
        alongside any temperature value that is printed. The glyph
        depends on the final temperature unit. If no conversion is
        made, then the glyph will be that of the base unit.

        This change only affects how following segments will be
        printed, it does not globally affect how all segments will
        be printed, or how segments before this argument will be
        printed. You can alternate between use/don't use by setting
        -ug y -tm -ug n -tx, for example, which will print the min
        core temp with a segment, and the max core temp without one.
        
        Valid boolean values are: true, yes, y, on, false, no, n, off

        E.g. whenever 53.32 is printed, if a temperature, will be
             printed as 53.32°C instead.

    
    --temp (-t) CORES...
        Prints the current temperature of the cores specified by
        CORES. If 'all' or '*' is given, display for all cores.

        If multiple cores are specified, then the temperatures will
        be printed in the order they were specified, and separated
        with a coma and space.

        Values are indicies, starting from 0. So to print 6 cores,
        the invocation would be: -t 0 1 2 3 4 5

        Which would output, e.g. (assuming no glyph, celcius):
            53.32, 53.32, 53.32, 53.32, 53.32, 53.32

        Most versatile when giving a single core number, and combining with -s
            Input: -s 'Core 5 = ' -t 4
            Output: Core 5 = 53.32


    --core-critical (-cC) CORES...
        Prints the critical temperature value for the given cores (see --temp)
        for acceptable values of CORES...

        Typicaly the same across all cores, between 90°C and 100°C
    

    --core-alarm (-ca) CORES...
        Prints the critical alarm value for the given cores (see --temp) for 
        acceptable values of CORES...

        This value indicates whether or not the core is considered to be at a
        critical temperature. The output can be true or false (per core).
";

pub fn exit_with_usage(code: i32) {
    println!("{}", USAGE);
    std::process::exit(code);
}
