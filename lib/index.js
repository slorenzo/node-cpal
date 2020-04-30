var addon = require('../native');

console.log('defaultInputDevice', addon.defaultInputDevice());
console.log('defaultOutputDevice', addon.defaultOutputDevice());
console.log('inputDevices', addon.inputDevices());
console.log('outputDevices', addon.outputDevices());
