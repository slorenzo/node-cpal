# node-cpal

It's an attempt to be a CPAL (Rust Cross-platform audio I/O library) bindings for Node.js with Neon.


## Install

To install, run:

`yarn add node-cpal`

or

`npm i node-cpal`

## Usage

To import the library use:

`import cpal from 'node-cpal'`

or

`const cpal = require('node-cpal')`

After that, you will be able to call some of the CPAL functions.

|  Functions  |  Description |
|---|---|
| `cpal.findAvailableHosts()`  | It provides access to the available audio devices on the system.  |
| `cpal.findDefaultInputDevice()`  | It provides access to the default input device.  |
| `cpal.findDefaultOutputDevice()`  | It provides access to the default output device.  |
| `cpal.findInputDevices()`  | It provides access to all input devices of the default host.  |
| `cpal.findOutputDevices()`  |  It provides access to all output devices of the default host. |

### Credits

Thanks to everyone involved in CPAL.





