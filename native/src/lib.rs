use neon::prelude::*;

mod funcs;

register_module!(mut cx, {
  cx.export_function("findAvailableHosts", funcs::available_hosts).unwrap();
  cx.export_function("findDefaultInputDevice", funcs::default_input_device).unwrap();
  cx.export_function("findDefaultOutputDevice", funcs::default_output_device).unwrap();
  cx.export_function("findInputDevices", funcs::input_devices).unwrap();
  cx.export_function("findOutputDevices", funcs::output_devices).unwrap();
  Ok(())
});
