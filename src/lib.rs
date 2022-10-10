use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use num::complex::Complex;
use web_sys::console;

struct Triangle {
    color : bool,
    a : Complex<f64>,
    b : Complex<f64>,
    c : Complex<f64>
}


// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    
    let performance = web_sys::window().unwrap().performance().expect("performance should be available");
    
    let height = document.body().unwrap().client_height();
    let width = document.body().unwrap().client_width();

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.translate(width as f64 / 2.0, height as f64 / 2.0);
    context.clear_rect(0.0, 0.0, width as f64, height as f64);


    let radius = height as f64 / 2.0;
    let start = performance.now();
    let triangle_set : Vec<Triangle> = penrose_type_2(10, radius);
    let end = performance.now();
    let duration : JsValue =  (end - start).into();
    console::log_2(&"Triangle calculation : ".into(), &duration);

    let start = performance.now();
    for t in &triangle_set {
        context.begin_path();
        if t.color {
            context.set_fill_style(&"#40C4FF".into());
        }
        else {
            context.set_fill_style(&"#64FFDA".into());
        }
        context.move_to(t.c.re, t.c.im);
        context.line_to(t.a.re, t.a.im);
        context.line_to(t.b.re, t.b.im);
        context.fill();
        context.stroke();
    }
    let end = performance.now();
    let duration : JsValue =  (end - start).into();
    console::log_2(&"Canvas calculation : ".into(), &duration);


    Ok(())
}

fn penrose_type_1(steps : u32, radius : f64) -> Vec<Triangle> {
    let gr : f64 = (1.0 + (5.0 as f64).sqrt()) / 2.0;
    let mut triangle_set : Vec<Triangle> = vec![];

    let init_set = 10;
    for i in 0..init_set {
        let iter = i as f64;
        let mut b = Complex::new(radius * ((2.0 * iter - 1.0) * f64::consts::PI / init_set as f64).cos() , radius * ((2.0 * iter - 1.0) * f64::consts::PI / init_set as f64).sin());
        let mut c = Complex::new(radius * ((2.0 * iter + 1.0) * f64::consts::PI / init_set as f64).cos() , radius * ((2.0 * iter + 1.0) * f64::consts::PI / init_set as f64).sin());
        
        if i % 2 == 0 {
            (b , c) = (c, b);
        }
        
        triangle_set.push( Triangle {
            color : true,
            a : Complex::new(0.0, 0.0),
            b : b,
            c : c
        })
    }


    for _ in 0..steps {
        let mut result : Vec<Triangle> = vec![];
        for t in &triangle_set {
            if t.color {
                let p = t.a + ((t.b - t.a) / gr);
                result.push(Triangle {
                    color : true,
                    a : t.c,
                    b : p,
                    c : t.b
                });
                result.push( Triangle{
                    color : false,
                    a : p,
                    b : t.c,
                    c : t.a
                });
            }
            else {
                let q = t.b + ((t.a - t.b) / gr);
                let r = t.b + ((t.c - t.b) / gr);
                result.push(Triangle {
                    color : false,
                    a : r,
                    b : t.c,
                    c : t.a
                });
                result.push(Triangle {
                    color : false,
                    a : q,
                    b : r,
                    c : t.b
                });
                result.push(Triangle {
                    color : true,
                    a : r,
                    b : q,
                    c : t.a
                });
            }
        }
        triangle_set = result;
    }
    return triangle_set;
}

fn penrose_type_2(steps : u32, radius : f64) -> Vec<Triangle> {

    let gr : f64 = (1.0 + (5.0 as f64).sqrt()) / 2.0;
    let mut triangle_set : Vec<Triangle> = vec![];

    let init_set = 10;
    for i in 0..init_set {
        let iter = i as f64;
        let mut a = Complex::new(radius * ((2.0 * iter - 1.0) * f64::consts::PI / init_set as f64).cos() , radius * ((2.0 * iter - 1.0) * f64::consts::PI / init_set as f64).sin());
        let mut c = Complex::new(radius * ((2.0 * iter + 1.0) * f64::consts::PI / init_set as f64).cos() , radius * ((2.0 * iter + 1.0) * f64::consts::PI / init_set as f64).sin());
        
        if i % 2 == 0 {
            (a , c) = (c, a);
        }
        
        triangle_set.push( Triangle {
            color : true,
            a : a,
            b : Complex::new(0.0, 0.0),
            c : c
        })
    }

    for _ in 0..steps {
        let mut result : Vec<Triangle> = vec![];
        for t in &triangle_set {
            if t.color {
                let q = t.a + ((t.b - t.a) / gr);
                let r = t.b + ((t.c - t.b) / gr);
                result.push(Triangle {
                    color : false,
                    a : r,
                    b : q,
                    c : t.b
                });
                result.push(Triangle {
                    color : true,
                    a : q,
                    b : t.a,
                    c : r
                });
                result.push(Triangle {
                    color : true,
                    a : t.c,
                    b : t.a,
                    c : r
                });
            }
            else {
                let p = t.c + ((t.a - t.c) / gr);
                result.push(Triangle {
                    color : true,
                    a : t.b,
                    b : p,
                    c : t.a
                });
                result.push( Triangle{
                    color : false,
                    a : p,
                    b : t.c,
                    c : t.b
                });
            }
        }
        triangle_set = result;
    }

    return triangle_set;
}


#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
