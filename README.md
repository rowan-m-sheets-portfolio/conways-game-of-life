# Conway's game of life for the micro:bit v2
An implementation of Conway's game of life built to run on the BBC micro:bit V2

## Authors
main.rs: Rowan Sheets\n
life.rs: provided by Bart Massey

## How to run
1. Make sure you have all the requirements to use embedded rust with the BBC micro:bit (instructions available
[here](https://docs.rust-embedded.org/discovery-mb2/index.html))
2. Run the command `cargo embed`
3. The A button on the microbit can be used to rapidly re-generate a new board state, the B button can be used to
invert the current board state

## Implementation
The program works by seeding a nanorand random number generator with a random number taken from the microbit's hardware
random number generator to ensure randomness without repeatedly using the costly hardware rng. This nanorand rng is then used
to create an inital board state which then has the rules of the game of life applied to it and is displayed at a framerate of 10 fps.
If the board is ever filled with all "dead" cells, it will wait 5 frames and then re-randomize.