use once_cell::sync::Lazy;
use std::{collections::HashMap, ffi::c_void};

// In-memory database key-value database.
// Each key represents a table.
// Values can be anything but in this project all values are vectors.
pub static mut DB: Lazy<HashMap<String, *mut c_void>> = Lazy::new(|| HashMap::new());

pub fn create_table<T>(name: impl ToString, value: T) {
    unsafe { DB.insert(name.to_string(), Box::into_raw(Box::new(value)).cast()) };
}

pub fn get_table<T>(name: impl ToString) -> Option<&'static T> {
    unsafe {
        if let Some(data) = DB.get(&name.to_string()) {
            Some(Box::leak(Box::from_raw(data.cast::<T>())))
        } else {
            None
        }
    }
}
pub fn get_table_mut<T>(name: impl ToString) -> Option<&'static mut T> {
    unsafe {
        if let Some(data) = DB.get_mut(&name.to_string()) {
            Some(Box::leak(Box::from_raw(data.cast::<T>())))
        } else {
            None
        }
    }
}

// Since we wrap things in a raw pointer we need to convert it back to Box
// so it gets dropped gracefully.
pub fn destroy_table<T>(name: impl ToString) {
    unsafe {
        // Type gets automatically dropped
        let _ = Box::from_raw(DB.get_mut(&name.to_string()).unwrap().cast::<T>());

        DB.remove(&name.to_string());
    }
}
