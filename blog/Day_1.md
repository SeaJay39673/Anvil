# Day 1

From what I can tell a terminal emulator is just a pseudo-terminal connection from the emulator application to whatever process is being executed. Then, the emulator application can just display the response from input however it wants. Sounds simple, but computer graphics is a lot of **heavy** boilerplate. And I've never dealt with PTY before.

I've found a rust crate called rust-pty. It supports async handling, so I'm going to start here and couple it with Tokio. Probably going to use WGPU for graphics but don't want to get into that for now.