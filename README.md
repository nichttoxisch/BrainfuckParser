Brainfuck parser
================

A [Brainfuck interpeter](https://en.wikipedia.org/wiki/Brainfuck) that fisrst parses the brainfuck program into C or Rust Source code wich is then compiled.
Just basic optimisation.

Usage on windows:
```bash
    cd BrainfuckParser_{C, Rust}
    ./run.bat
```

Benchmark with the [Mandelbrotset](https://github.com/ErikDubbelboer/brainfuck-jit/blob/master/mandelbrot.bf) on windows:
=========================================================================================================================

# without compiling

PS X:\dev\0021_brainfuckConverterRust-rust> Measure-Command{ .\out\brainfuck.exe }

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 1
Milliseconds      : 0
Ticks             : 10004035
TotalDays         : 1,1578744212963E-05
TotalHours        : 0,000277889861111111
TotalMinutes      : 0,0166733916666667
TotalSeconds      : 1,0004035
TotalMilliseconds : 1000,4035


PS X:\dev\0022_brainfuckConverterC-rust> Measure-Command{ .\out\brainfuck.exe }

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 883
Ticks             : 8831692
TotalDays         : 1,02218657407407E-05
TotalHours        : 0,000245324777777778
TotalMinutes      : 0,0147194866666667
TotalSeconds      : 0,8831692
TotalMilliseconds : 883,1692


# with compiling

PS X:\dev\0021_brainfuckConverterRust-rust> Measure-Command{ .\run.bat }

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 16
Milliseconds      : 573
Ticks             : 165731548
TotalDays         : 0,000191818921296296
TotalHours        : 0,00460365411111111
TotalMinutes      : 0,276219246666667
TotalSeconds      : 16,5731548
TotalMilliseconds : 16573,1548

PS X:\dev\0022_brainfuckConverterC-rust> Measure-Command{ .\run.bat }

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 3
Milliseconds      : 522
Ticks             : 35223103
TotalDays         : 4,07674803240741E-05
TotalHours        : 0,000978419527777778
TotalMinutes      : 0,0587051716666667
TotalSeconds      : 3,5223103
TotalMilliseconds : 3522,3103

