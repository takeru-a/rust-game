use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>() // HtmlCanvansElementキャストしている
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>() // HtmlCanvansElementキャストしている
        .unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);
        let image = web_sys::HtmlImageElement::new().unwrap();
        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                success_tx.send(Ok(()));
            }
        });

        let error_callback = Closure::once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                error_tx.send(Err(err));
            }
        });

        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        callback.forget();

        image.set_src("Idle01.png");

        // Receiver
        success_rx.await;
        context.draw_image_with_html_image_element(&image, 0.0, 0.0);
        sierpinski(
            &context,
            [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)],
            5,
            (0, 0, 0),
        );
        context.draw_image_with_html_image_element(&image, 0.0, 0.0);

        let json = fetch_json("rhb.json")
            .await
            .expect("Could not fetch rhb.json");

        let sheet: Sheet = json
            .into_serde()
            .expect("Could not convert rhb.json into a Sheet structure");

            let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
            let success_tx = Rc::new(Mutex::new(Some(success_tx)));
            let error_tx = Rc::clone(&success_tx);
            let image = web_sys::HtmlImageElement::new().unwrap();
            let callback = Closure::once(move || {
                if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                    success_tx.send(Ok(()));
                }
            });
    
            let error_callback = Closure::once(move |err| {
                if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                    error_tx.send(Err(err));
                }
            });
    
            image.set_onload(Some(callback.as_ref().unchecked_ref()));
            image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    
            callback.forget();
    
            image.set_src("rhb.png");
    
            // Receiver
            success_rx.await;

            let sprite = sheet.frames.get("Run (1).png").expect("Cell not found");
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                sprite.frame.x.into(),
                sprite.frame.y.into(),
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                300.0,
                300.0,
                sprite.frame.w.into(),
                sprite.frame.h.into(),
               );
    });
    Ok(())
}

async fn fetch_json(json_path: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(json_path)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;

    wasm_bindgen_futures::JsFuture::from(resp.json()?).await
}

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
) {
    let [top, left, right] = points;
    let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));
    context.move_to(top.0, top.1); //上の頂点
    context.begin_path();
    context.line_to(left.0, left.1); // 左下の頂点
    context.line_to(right.0, right.1); // 右下の頂点
    context.line_to(top.0, top.1); // 上の頂点に戻る
    context.close_path();
    context.stroke();
    context.fill();
}

fn sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    depth: u8,
    color: (u8, u8, u8),
) {
    draw_triangle(&context, points, color);
    let depth = depth - 1;
    let [top, left, right] = points;

    if depth > 0 {
        let mut rng = thread_rng();
        let next_color = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );
        let left_middle = ((top.0 + left.0) / 2.0, (top.1 + left.1) / 2.0);
        let right_middle = ((top.0 + right.0) / 2.0, (top.1 + right.1) / 2.0);
        let bottom_middle = (top.0, right.1);
        sierpinski(
            &context,
            [top, left_middle, right_middle],
            depth,
            next_color,
        );
        sierpinski(
            &context,
            [left_middle, left, bottom_middle],
            depth,
            next_color,
        );
        sierpinski(
            &context,
            [right_middle, bottom_middle, right],
            depth,
            next_color,
        );
    }
}
