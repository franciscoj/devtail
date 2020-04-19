# Devtail

First and foremost... Don't use this! It is just an experiment to learn some
rust and allow me to experiment with some ideas for devtools. It is not
maintained it will probably never be finished and it has no functionalities
that are atractive to anybody but me.

Now... That said. `devtail` is meant to become a tool to substitute `tail -f
log/development.log` for me while I'm working on a rails project.

I want it to group the requests on that log by order of appearance, collapsing
them to a single line representation allowing me to uncollapse them when I want
some more information.

I would also like to have some other dev niceties like:

* Allowing to see only the requests to a given controller or filtering in any
    other way.
* Marking requests that do more than X DB queries or that are too expensive (to
    help debug N+1s)
* Having a nice wrapping for error backtraces.
* Good keyboard shortcuts.
* Others that I don't know yet!

For now this is just a very poor and very silly substitute to `tail -f` :shrug:
