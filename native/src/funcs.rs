use neon::prelude::*;
use cpal:: traits:: { DeviceTrait, HostTrait };

pub fn default_input_device(mut cx: FunctionContext) -> JsResult<JsObject> {
  match cpal::default_host().default_input_device() {
    Some(device) => {
      let obj: Handle<JsObject> = JsObject::new(&mut cx);

      let name = cx.string(device.name().unwrap());

      obj.set(&mut cx, "name", name).unwrap();

      Ok(obj)
    },
    None => cx.throw_error("No default input device"),
  }
}

pub fn default_output_device(mut cx: FunctionContext) -> JsResult<JsObject> {
  match cpal::default_host().default_output_device() {
    Some(device) => {
      let obj: Handle<JsObject> = JsObject::new(&mut cx);

      let name = cx.string(device.name().unwrap());

      obj.set(&mut cx, "name", name).unwrap();

      Ok(obj)
    },
    None => cx.throw_error("No default output device"),
  }
}


pub fn input_devices(mut cx: FunctionContext) -> JsResult<JsArray> {
  let host = cpal:: default_host();

  let devices = host.input_devices().unwrap();

  let js_array: Handle<JsArray> = cx.empty_array();

  for (device_index, device) in devices.enumerate() {
    let obj: Handle<JsObject> = JsObject::new(&mut cx);

    let index = cx.number(device_index as f64);
    let name = cx.string(device.name().unwrap());

    obj.set(&mut cx, "id", index).unwrap();
    obj.set(&mut cx, "name", name).unwrap();

    let _ = js_array.set(&mut cx, device_index as u32, obj);
  }

  Ok(js_array)
}

pub fn output_devices(mut cx: FunctionContext) -> JsResult<JsArray> {
  let host = cpal:: default_host();

  let devices = host.output_devices().unwrap();

  let js_array: Handle<JsArray> = cx.empty_array();

  for (device_index, device) in devices.enumerate() {
    let obj: Handle<JsObject> = JsObject::new(&mut cx);

    let index = cx.number(device_index as f64);
    let name = cx.string(device.name().unwrap());

    obj.set(&mut cx, "id", index).unwrap();
    obj.set(&mut cx, "name", name).unwrap();

    let _ = js_array.set(&mut cx, device_index as u32, obj);
  }

  Ok(js_array)
}