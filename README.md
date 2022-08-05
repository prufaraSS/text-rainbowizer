lazily written rust program which makes animated colorized output

proper usage of data generated in text rainbowizer as example:

![cmd_4fdAf9LUGr](https://user-images.githubusercontent.com/19390500/183110609-b0323c00-660c-4780-b6af-f11a1cac0530.gif)

there's some options provided so you can make little uncommon shapes of rainbow

![image](https://user-images.githubusercontent.com/19390500/183110956-a8b73ae3-718e-403e-946f-b2ca9cc1aaf3.png)

can't open the program? probably no 'input.txt' file found otherwise the problem is with your OS

program crashed when all settings provided? probably your PC is out of memory, so restart it. anyways, error caused by buffers (can't flush, can't write) or by file system (can't write output.txt)

### how to use generated data?

if you open output.txt you can see a very messy structure of some numbers and strange symbols, but there's one similarity in this mess - a blank lines at every nth positions - this is a frame divider, you can use it to split whole file in a vector of frames.
