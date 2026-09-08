# Day 2

Okay, so I got the basics working with pty. I was worried about being able to simultaneously read/write with the master but conveniently enough Tokio is able to split it into read/write halves. So now I have a read task that prints it's result to the console, and a write half that reads it's input from user. Not entirely sure where to go from here... Should I dive GUI development? We will see.

