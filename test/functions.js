const cpal = require('..');

try {
	const availableHosts = cpal.findAvailableHosts();
	console.log('availableHosts', availableHosts);

	const defaultInputDevice = cpal.findDefaultInputDevice();
	console.log('defaultInputDevice', defaultInputDevice);

	const defaultOutputDevice = cpal.findDefaultOutputDevice();
	console.log('defaultOutputDevice', defaultOutputDevice);

	const inputDevices = cpal.findInputDevices();
	console.log('inputDevices', inputDevices);

	const outputDevices = cpal.findOutputDevices();
	console.log('outputDevices', outputDevices);

	process.exit(0);
} catch (e) {
	console.error(e);
	process.exit(1);
}
