Brainfuck parser
================

A [Brainfuck interpeter](https://en.wikipedia.org/wiki/Brainfuck) that fisrst parses the brainfuck program into C or Rust Source code wich is then compiled.
Just basic optimisation.

Usage on windows:
```bash
    cd BrainfuckParser_{C, Rust}
    ./run.bat
```

#Benchmark with the [Mandelbrotset](https://github.com/ErikDubbelboer/brainfuck-jit/blob/master/mandelbrot.bf) on windows:

# without compiling

## Rust implementation
PS X:\dev\0021_brainfuckConverterRust-rust> Measure-Command{ .\out\brainfuck.exe }

Seconds           : 1
Milliseconds      : 0
Ticks             : 10004035

## C Implementation
PS X:\dev\0022_brainfuckConverterC-rust> Measure-Command{ .\out\brainfuck.exe }

Seconds           : 0
Milliseconds      : 883
Ticks             : 8831692


# with compiling

## Rust implementation
PS X:\dev\0021_brainfuckConverterRust-rust> Measure-Command{ .\run.bat }

Seconds           : 16
Milliseconds      : 573
Ticks             : 165731548


## C Implementation
PS X:\dev\0022_brainfuckConverterC-rust> Measure-Command{ .\run.bat }

Seconds           : 3
Milliseconds      : 522
Ticks             : 35223103


