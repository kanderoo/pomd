# pomd: a command line pomdoro timer

`pomd` is a fully functional [pomodoro technique](https://en.wikipedia.org/wiki/Pomodoro_Technique) timer for the command line.
It defaults to a 25-5-30 work cycle, though that can be customized through command line arguments.

Here it is in action:

![](./resources/demo.gif)

## Features

* Custom Work/Short Break/Long Break length support
* Task logging support with the `--log` flag

## Roadmap

- [ ] Add support for a persistant configuration file
- [x] Add task logging
- [ ] Add database to task logging
- [ ] Add Taskwarrior integration
- [ ] Move to an async runtime
- [ ] Make pomodoro complete sound user-configurable
- [ ] Change logging format to JSON

## Architecture

In case you're digging through the code, here's how it is laid out.

The program roughly follows MVC. `App` is the Model, `Tui` is the View, `EventHandler` is the Controller.
MVC isn't _exactly_ followed in the sense that the Model does not explicitly tell the View when it is changed. Rather, all events change the Model, so after an Event is handled the View grabs state from the Model and outputs appropriately. I'm honestly not too sure if that's good practice or not in Rust (encapsulation is broken), so if anyone wants to correct me on that feel free.

The program runs in 3 threads:

1. The (display, wait for event, handle event) loop
2. A KeyListener loop that listens for key press events and notifies the Controller
3. A Timer loop that notifies the Controller after every second has passed