# Day 2

Okay, so I got the basics working with pty. I was worried about being able to simultaneously read/write with the master but conveniently enough Tokio is able to split it into read/write halves. So now I have a read task that prints it's result to the console, and a write half that reads it's input from user. Not entirely sure where to go from here... Should I dive GUI development? We will see.

So, I spoke with my architect (ChatGPT) and it suggested I hold off on doing GUI development right away. Apparently I should work on "Terminal Core," basically creating a terminal package that does all the pty parsing and state management of the terminal. This *shouldn't* be tightly coupled into pty or rendering, which I agree. Although Terminal Core is quite a lot of stuff to work on. So I'm going to start off with a data model and some unit tests. 

I realized after my most recent commit that I'm basically just creating a single terminal emulator. I want something closer to Terminator. Going to have to revise how this looks architecturally... Stay tuned for my next commit!