use engine_core::tool_registry;

fn main() {
    // Register tools
    tool_registry::register_tool("Brush");
    tool_registry::register_tool("Eraser");
    tool_registry::register_tool("Selector");
    println!("Registered tools: {:?}", tool_registry::get_tools());

    // Remove a tool
    tool_registry::remove_tool("Eraser");
    println!("After removal: {:?}", tool_registry::get_tools());

    // Check if a tool is registered
    println!("Is 'Brush' registered? {}", tool_registry::is_tool_registered("Brush"));
    println!("Is 'Eraser' registered? {}", tool_registry::is_tool_registered("Eraser"));
}
