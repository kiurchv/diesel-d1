use wasm_bindgen::{JsCast, JsValue};
use js_sys::Uint8Array;

pub struct D1Value {
    _row: JsValue
}



impl D1Value {
    pub fn new(row: JsValue) -> Self {
        Self { _row: row }
    }
    
    pub (crate) fn read_string(&self) -> String {
        self._row.as_string().unwrap()
    }

    pub (crate) fn read_bool(&self) -> bool {
        self._row.as_bool().unwrap()
    }

    /// JS numbers are always f64, this might cause precision issues when crossing boundaries
    pub (crate) fn read_number(&self) -> f64 {
        self._row.as_f64().unwrap()
    }

    pub (crate) fn check_null(&self) -> bool {
        // not sure if undefined works
        self._row.is_null() || self._row.is_undefined() 
    }

    pub (crate) fn read_blob(&self) -> Vec<u8> {
        if !self._row.is_instance_of::<Uint8Array>() {
            panic!("JSValue is not uint8arrary");
        }
        // hummm hopefully _row is reference counted ahah
        Uint8Array::from(self._row.clone()).to_vec()
    }
}