mod utils;

use wasm_bindgen::prelude::*;

extern crate web_sys;

#[macro_export]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


use std::collections::BTreeMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-compress!");
}

#[wasm_bindgen]
pub struct WasmCompress {
    orig_string: String,
    result_string: String,
    f: f64,
    byte_len: u8,
    interv: BTreeMap<u8, (f64, f64) >, //because we need a total order
}

#[wasm_bindgen]
impl WasmCompress {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmCompress {
        WasmCompress {
            orig_string: String::new(),
            result_string: String::new(),
            f: 0.0,
            byte_len: 0,
            interv: BTreeMap::new(),
        }
    }

    pub fn get_tree_string(&self) -> String {
        format!("{:?}", self.interv)
    }

    pub fn get_string(&self) -> String {
        self.orig_string.clone()
    }

    pub fn get_result_string(&self) -> String {
        self.result_string.clone()
    }

    pub fn set_string(&mut self, s: &str) {
        self.orig_string.clear();
        self.orig_string.push_str(&s);
        self.analyze();
        self.compress();
        self.uncompress();
    }

    pub fn get_result_float(&self) -> f64 {
        log!("returning {}", self.f);
        self.f
    }
}

impl WasmCompress {

    fn analyze(&mut self) {
        let bytes : &[u8] = self.orig_string.as_bytes(); 
        self.byte_len = bytes.len() as u8; //won't get string bigger than 255 bytes anyway, due to float precision, it won't get much than 15 utf-8 chars
        self.interv.clear();

        for c in bytes {

            //entry() -> Entry which is an enum. If OccupiedEntry and_modify will add 1
            //or if VacantEntry will insert value 1.0. Yes, and_modify seems magic
            // '.2' refers to index 2 of tuple
            self.interv.entry(*c).and_modify(|value| {(*value).1 += 1.0} ).or_insert((0.0, 1.0));
        }

        let mut acc = 0.0;
        let mut prev_acc = 0.0;
        let lenf = self.byte_len as f64;

        for value in self.interv.values_mut() { //return &mut (f64, f64)
            acc += value.1 / lenf;
            *value = (prev_acc, acc);
            log!("({} {})", value.0, value.1);
            prev_acc = acc;
        }
    }

    fn compress(&mut self) {
        let bytes : &[u8] = self.orig_string.as_bytes(); 
        let mut b_inf : f64 = 0.0;
        let mut b_sup : f64 = 1.0;
        let mut delta : f64;

        for symbol in bytes {
            if let Some(bornes) = self.interv.get(symbol) {
                delta = b_sup - b_inf;
                b_sup = b_inf + delta * bornes.1;
                b_inf = b_inf + delta * bornes.0;
                log!("delta {} = {} - {} -> {}", delta, b_sup, b_inf, symbol);
            } else {
                log!("Couldn't find first char in interv hash map, something must have been gone wrong");
                panic!("Exited due to previous error")
            }
        }

        self.f = b_inf + (b_sup - b_inf) / 2.0; //TODO find smallest floating number representation
    }

    fn uncompress(&mut self) {
        self.result_string.clear();
        let mut result_str: Vec<u8> = Vec::new();

        let mut fval = self.f;
        for _ in 0..self.byte_len {
            for key_val in &self.interv {

                let b_inf = (key_val.1).0;
                let b_sup = (key_val.1).1;

                if b_inf < fval && fval < b_sup {
                    log!("{} < fval = {} < {} -> {}", b_inf, fval, b_sup, *key_val.0);
                    result_str.push(*key_val.0);
                    
                    fval = (fval - b_inf) / (b_sup - b_inf);
                    log!("new fval = {}", fval);
                    break;
                }
            }
        }

        unsafe {
            self.result_string = String::from_utf8_unchecked(result_str);
        }
    }
}


