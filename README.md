# Minimal test to demonstrate timing curiosity on STM32H7

I’m seeing something I don’t understand with actual `Timer::after_ticks()` delays being sometimes much, much longer than requested in debug builds.  I see slightly longer delays in release builds, but nothing like debug builds, and only for certain values of delay. Note: I’m using `tick-hz-1_000_000`.

The test just times 1000 delays of each size and spits it out on the UART afterwards so I can analyze it in Octave.

It is the 30ms of extra delay only on the magic values between 63 and 71 in debug builds that seems so odd. This effect seems very repeatable to me, though changing the test changes the magic values.

![graphs](/Screenshot%20from%202023-12-14%2016-40-09.png)
