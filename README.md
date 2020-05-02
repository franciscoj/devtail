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

- [ ] Allowing to see only the requests to a given controller or filtering in
      any other way (only GET requests or only PUT, PATCH and POST requests)
- [ ] Marking requests that do more than X DB queries or that are too expensive
      (to help debug N+1s or other kind of DB issues)
- [ ] Having a nice wrapping for error backtraces.
- [ ] Good keyboard shortcuts. It all has to work TUI style, a la VIM or emacs
      or you know... Terminal stuff.
- [X] Allow to receive logs either from a file `tail -f` or from a pipe (do you
      do that with standard input? :shrug:)
- [ ] Allow to configure some of those so that It is not only useful to rails
      projects.

For now this is just a very poor and very silly substitute to `tail -f` :shrug:

As I build it I realize that some stuff could be done better:

- [ ] Error management is quite poor. I'm probably abusing `.unwrap()` and or
      `?`
- [ ] There has to be a way to test `TailF` and `TailS` on integration but I
      still don't know  how to do it.
- [ ] Letting the `Log` structure grow without limit is probably a bad idea and
      it might have to work as a stack with a given max size.
