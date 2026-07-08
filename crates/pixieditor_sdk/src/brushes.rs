use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use abi::add_tool_to_toolset_with_config;
use crate::abi;

pub fn register_tool(pixi_file: &[u8], config_file: &[u8]) {
    unsafe {
        abi::register_brush_tool(
            pixi_file.as_ptr(),
            pixi_file.len() as i32,
            config_file.as_ptr(),
            config_file.len() as i32,
        );
    }
}

pub fn add_to_toolset(tool_name: &str, toolset_name: &str, at_index: i32) {
    unsafe {
        abi::add_tool_to_toolset(
            tool_name.as_ptr(),
            tool_name.len() as i32,
            toolset_name.as_ptr(),
            toolset_name.len() as i32,
            at_index,
        );
    }
}

pub fn add_to_toolset_with_config(tool_name: &str, toolset_name: &str, at_index: i32, config_json: &str) {
    unsafe {
        add_tool_to_toolset_with_config(
            tool_name.as_ptr(),
            tool_name.len() as i32,
            toolset_name.as_ptr(),
            toolset_name.len() as i32,
            at_index,
            config_json.as_ptr(),
            config_json.len() as i32,
        );
    }
}

pub fn register_brushes_data_source(name: &str, brushes: &[u8])
{
    unsafe{
        abi::register_brush_data_source(name.as_ptr(), name.len() as i32, brushes.as_ptr(), brushes.len() as i32);
    }
}

pub trait BrushDataSource {
    fn name(&self) -> &str;
    fn get_brushes(&self) -> Vec<Vec<u8>>;
}

static BRUSH_DATA_SOURCES: OnceLock<Mutex<HashMap<i32, Box<dyn BrushDataSource + Send>>>> = OnceLock::new();

fn brush_data_sources() -> &'static Mutex<HashMap<i32, Box<dyn BrushDataSource + Send>>> {
    BRUSH_DATA_SOURCES.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_data_source(data_source: Box<dyn BrushDataSource + Send>) {
    let mut sources = brush_data_sources()
        .lock()
        .unwrap();

    if sources.values().any(|x| std::ptr::eq(&**x, &*data_source)) {
        panic!("Data source is already registered.");
    }

    let brushes = data_source.get_brushes();

    let mut data = Vec::new();

    data.extend_from_slice(&(brushes.len() as i32).to_le_bytes());

    for brush in brushes {
        data.extend_from_slice(&(brush.len() as i32).to_le_bytes());
        data.extend_from_slice(&brush);
    }

    let name = data_source.name();

    let handle = unsafe {
        abi::register_brush_data_source(
            name.as_ptr(),
            name.len() as i32,
            data.as_ptr(),
            data.len() as i32,
        )
    };

    sources.insert(handle, data_source);
}