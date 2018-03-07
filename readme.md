# Sanitize UTF8

What it says on the tin.

This program is so simple I'm ashamed to call it a program, but it has taught
me a little rust, and killed a few hours.

i3bar was giving me issues with incorrect UTF8 from conky, so I dished up
something that I could put in a unix pipe that could clean up the stream. Turns
out Rust's IO does this for me already, so all I'm doing is reading input
character-wise, and writing them right back out, ignoring any errors. Errors in
this case include invalid UTF8, so there's that. Thanks Rust!


## Build

Inside the project root directory run:

`$ cargo build`
