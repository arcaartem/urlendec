% URLENDEC(1)  urlendec 0.1.1
% Arca Artem
% January 2023

# NAME
urlendec - simple URL encoder/decoder

# SYNOPSIS
**urlendec** [*OPTION*]

# DESCRIPTION
**urlendec** allows encoding or decoding a string argument or list of strings from a file. Running **urlendec** without any command line parameters causes **urlendec** to read input from STDIN and write to STDOUT.

# OPTIONS
**-d**, **--decode**
: Decode the input. By default, **urlendec** encodes the input. Use this option to decode instead.

**-h**, **--help**
: Displays a friendly help message.

**-i**, **--input-file** _filename_
: Read string(s) from the input file. Defaults to _-_, (i.e. _STDIN_). This option cannot be used with **--string**.

**-o**, **--output-file** _filename_
: Write the resulting string(s) to the output file. Defaults to _-_ (i.e. _STDOUT_).

**-s**, **--string** _string_
: Encode or decode provided string _string_. This option cannot be used with **--input-file**.

**-V**, **--version**
: Displays the software version.

# EXAMPLES
**urlendec -s "Hello, world!"**
: Encodes the string _Hello, world!_. The output, _Hello%2C%20world%21_, will be printed out to _STDOUT_.

**urlendec -i input.txt -o output.txt**
: Encodes each line in _input.txt_ and writes them to _output.txt_ one by one.

# EXIT VALUES
**0**
: Success

**1**
: Invalid option

# COPYRIGHT
Copyright (c) 2023 Arca Artem. License MIT. This is free software: you are free to change and redistribute it. There is NO WARRANTY, to the extent permitted by law.
