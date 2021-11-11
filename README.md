Brainfuck parser
================

A [Brainfuck interpeter](https://en.wikipedia.org/wiki/Brainfuck) that fisrst parses the brainfuck program into C or Rust Source code wich is then compiled.
Just basic optimisation.

Usage on windows:
```bash
    cd BrainfuckParser_{C, Rust}
    ./run.bat
```

# Benchmark with the [Mandelbrotset](https://github.com/ErikDubbelboer/brainfuck-jit/blob/master/mandelbrot.bf) on windows:

### without compiling

#### Rust implementation
Seconds           : 1\
Milliseconds      : 0\
Ticks             : 10004035

-> C Implementation\
    Seconds           : 0\
    Milliseconds      : 883\
    Ticks             : 8831692


### with compiling

#### Rust implementation
Seconds           : 16\
Milliseconds      : 573\
Ticks             : 165731548


#### C Implementation
Seconds           : 3\
Milliseconds      : 522\
Ticks             : 35223103


