use neon::prelude::*;

mod funcs;

register_module!(mut cx, {
  cx.export_function("defaultInputDevice", funcs::default_input_device).unwrap();
  cx.export_function("defaultOutputDevice", funcs::default_output_device).unwrap();
  cx.export_function("inputDevices", funcs::input_devices).unwrap();
  cx.export_function("outputDevices", funcs::output_devices).unwrap();
  Ok(())
});
