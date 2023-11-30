use wasm_bindgen::prelude::*;
use std::convert::TryInto;

#[wasm_bindgen(start)]
fn main() {
    let document = web_sys::window().unwrap()
        .document().unwrap();

    let canvas = document.get_element_by_id("viewport").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap() // Unwrap result
        .unwrap() // Unwrap option
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let width = canvas.width();
    let height = canvas.height();
    let mut data = vec![0; TryInto::<usize>::try_into(width * height).unwrap() * 4];
    for i in 0..(width * height) {
        let index = TryInto::<usize>::try_into(i).unwrap();
        data[index * 4 + 0] = 255;
        data[index * 4 + 1] = 0;
        data[index * 4 + 2] = 0;
        data[index * 4 + 3] = 0;
    }

    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&data), width, height
    ).unwrap();

    let _ = context.put_image_data(&image_data, 0.0, 0.0);
}
