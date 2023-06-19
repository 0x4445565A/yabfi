# YABFI
Yet Another Brain F*ck Interpreter is a Rust experiment for executing BF code.  It has two run modes interpreter mode and STDIN mode.  Interpreter mode allows you to write and execute BF with some additional tools (h, q, #).  STDIN mode allows you to pipe BF code straight in, doing this disables the extra tools because I want to support the standard BF experience.

# Build
`cargo build`


# Usage
Here is an example of piping in the example hello world program into the interpreter.
```
cat example.bf | ./target/debug/yabfi
```

Using interpreter mode with usage.
```
╰─$ ./target/debug/yabfi     

No STDIN detected, using interpreter CLI...
    __   _____  ____________ _____ 
    \ \ / / _ \ | ___ \  ___|_   _|
     \ V / /_\ \| |_/ / |_    | |  
      \ /|  _  || ___ \  _|   | |  
      | || | | || |_/ / |    _| |_ 
      \_/\_| |_/\____/\_|    \___/ 
    Yet Another Brain F*ck Interpreter
                         (q)uit (h)elp
        
>>> h   
Wiki: https://esolangs.org/wiki/Brainfuck

Non-standard BF symbols (Only in interpret mode)
---------
h: Print this help
q: Quit the program
#: Print the entire memory buffer

Standard BF symbols
---------
>: Moves pointer to the right
<: Moves pointer to the left
+: Increment the memory cell at the pointer
-: Decrement the memory cell at the pointer
.: Print the character signified by the cell at the pointer
,: Input a number and store it in the cell at the pointer
[: Jump past the matching ] if the cell at the pointer is 0
]: Jump back to the matching [ if the cell at the pointer is nonzero
```