# Tools Programmer Homework

Your mission: implement a web service that takes in a set of bytes as input and outputs a
MOS 6502 microprocessor family disassembly of those bytes as output. For example, assuming the
following input bytes:

```
a9 bd a0 bd 20 28 ba
```

The following output (or similar) should be produced:

```
0x0000 a9 bd        LDA #$bd
0x0002 a0 bd        LDY #$bd
0x0004 20 28 ba     JSR $ba28
```

You can, based on your own preferences, change the format of the output, but the origin address and bytes used in
disassembly should always be visible. You can deal with unknown opcodes however you deem appropriate
(outputting just the hex bytes is sufficient, for example). You should assume the input begins at address 0 (or
you can make this configurable in some fashion). You should always try to consume all of the input data (or you
can make this configurable too!). There are some example binaries in the `test-bin/` directory of
the source tree that contain valid 6502 code that can be used for testing.

Information on the MOS 6502 instruction set is available all over the internet, but a handy instruction set guide
that should provide sufficient information is available at https://www.masswerk.at/6502/6502_instruction_set.html

A simple testing scaffold is provided for testing and development convenience that should serve as a starting point.
You can `cargo run` the scaffold in one terminal to start up the testing web server and then run `cargo test` in
another to send a test request to it. You can modify the existing test and add new ones as desired.

You can spend as much time as you feel is required for a baseline implementation. The idea is not to find out if you
can read an opcode listing and implement instruction decoding based on that - we can assume this is the case. Instead,
try to sketch out an architecture and abstractions for your program that are easy to understand and possibly extend.
Be sure to consider things like "what if we wanted to implement support for additional architectures?"

If you can find a way to produce proper disassembly from the binary data without implementing a full disassembler,
feel free to do so as well.

Incomplete solutions are also better than nothing! Don't worry if you don't have time to implement decoding for all
instructions or are missing some undocumented ones! If you find the task to be unexpectedly difficult, don't worry!
If you can provide some initial code and are prepared to explain your implementation idea, that is fine as well.

Don't hesitate to use pseudocode in places where you feel a lot of boilerplate or obvious implementations are required.

# My notes

All in all this was a really fun exercise! I've never done anything with 6502
assembly before and first I was a bit scared at first but it turned out to be
more pleasant of an exercise than I thought, honestly. Writing rust is real
refreshing after years of Typescript...

You can run the tests by just running `cargo test`, it automatically starts the
test server, no need to run `cargo run` first.

## Major things I left out

I didn't bother adding labels, I had enough of looking at the disassemble
method working on this over the weekend! Everything is just raw memory
addresses in the output. With quick googling I would probably do this with a
second first-pass, looping through the data and collecting the relevant memory
addresses if we encounter a branching or jump instruction, then include them in
the actual disassembly

Second major thing I didn't bother with parsing the data section, now
everything is treated as code! Most actual disassembled 6502 programs would
then have big chunks of invalid opcodes or some other nonsense. It seems like
quite the endeavor, but here's some things I would probably try out first

- From the starting point, if we don't do any jumps or branching we're likely
  still parsing code
- if we encounter LDA/sta or similar load/store instructions, there's a good
  change we're looking at data
- If we do jmp or branch somewhere, it's likely code. Especially if the jump is
  unconditional!
- Maybe we can try to figure out if we're dealing with lines of ASCII or arrays
  by comparing sequences of bytes for similarity (seems like a stretch)

But yeah it's tricky! To alleviate the problem I'm allowing the user to provide
the start and end address of the data they want to disassemble. This way they
can just disassemble the code section and then the data section separately.

## Other notes

There's a branch with JSON schema validation included for validating the opcode files,
but I left that out of `main` since after writing it feels a bit overkill to be honest, since
we're using rust, not shitty dynamic language. Just the web plebeian in me!

## Sources

https://www.masswerk.at/6502/disassembler.html
https://www.youtube.com/watch?v=mR1G9ZA2UfQ
https://www.awsm.de/blog/pydisass/
https://6502disassembly.com/on-disassembly.html
https://codeburst.io/an-introduction-to-6502-assembly-and-low-level-programming-7c11fa6b9cb9
https://skilldrick.github.io/easy6502/

Copy-paste to/from chatGPT :call_me:
