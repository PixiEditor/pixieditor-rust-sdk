use crate::abi;

pub fn register_brush(pixi_file: &[u8], config_file: &[u8]) {
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

