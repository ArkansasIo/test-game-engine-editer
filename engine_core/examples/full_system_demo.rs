use engine_core::*;

fn main() {
    // Core systems
    rendering::initialize();
    rendering::render_frame();
    rendering::set_viewport(1280, 720);
    rendering::load_texture("assets/tex.png");

    physics::initialize();
    physics::step_simulation(0.016);
    physics::add_rigid_body(1);
    physics::remove_rigid_body(1);

    input::initialize();
    input::poll_events();
    input::is_key_pressed("Space");
    input::mouse_position();

    audio::initialize();
    audio::play_sound(1);
    audio::stop_sound(1);
    audio::set_volume(0.5);
    audio::pause_all();
    audio::resume_all();

    scripting::initialize();
    scripting::run_script("print('Hello')");
    scripting::register_function("my_func");

    // ECS
    let entity = ecs::create_entity();
    ecs::add_component(&entity, "Transform");
    ecs::remove_component::<&str>(&entity);
    ecs::destroy_entity(&entity);

    // Scene
    scene::load_scene("TestScene");
    scene::save_scene("TestScene");
    scene::switch_scene("OtherScene");

    // UI
    ui::show_ui();
    ui::add_widget("Button");
    ui::remove_widget("Button");

    // Game logic
    game_logic::update_game_logic();
    game_logic::add_logic_system("AI");
    game_logic::remove_logic_system("AI");

    // Tools & Menus
    tools::open_tool("Brush");
    tools::close_tool("Brush");
    tools::list_tools();
    menus::show_menu("File");
    menus::add_menu_item("File", "New");
    menus::remove_menu_item("File", "New");

    // Registries
    tool_registry::register_tool("Brush");
    println!("Registered tools: {:?}", tool_registry::get_tools());
    tool_registry::remove_tool("Brush");
    println!("Registered tools after removal: {:?}", tool_registry::get_tools());
    println!("Is 'Brush' registered? {}", tool_registry::is_tool_registered("Brush"));

    feature_registry::register_feature("Lighting");
    println!("Registered features: {:?}", feature_registry::get_features());

    logic_system::add_logic_statement("if player.health < 0 { game_over(); }");
    logic_system::run_logic();
    statement_system::add_statement("print('Statement')");
    statement_system::execute_statements();

    // Blueprints
    blueprints::create_blueprint("PlayerBP");
    blueprints::nodes::register_node_type("Add");
    blueprints::graph::create_graph("MainGraph");
    blueprints::runtime::execute_blueprint("PlayerBP");

    // Editor & Runtime
    editor::launch_editor();
    editor::menus::add_menu("Edit");
    editor::tools::add_tool("Move");
    runtime::start_game();

    // Plugins & Modules
    plugins::load_plugin("PhysicsPlugin");
    modules::register_module("RenderingModule");

    // Asset system
    asset::load_asset("assets/hero.png");
    asset::unload_asset("assets/hero.png");
    texture::load_texture("assets/tex.png");
    model::load_model("assets/hero.obj");
    material::load_material("assets/hero.mat");
    animation::load_animation("assets/hero.anim");
    skeleton::load_skeleton("assets/hero.skel");
    sound::load_sound("assets/hero.wav");
    font::load_font("assets/font.ttf");
    shader::load_shader("assets/shader.glsl");

    // Math system
    let v2 = vec2::Vec2::new(1.0, 2.0);
    let v3 = vec3::Vec3::new(1.0, 2.0, 3.0);
    let v4 = vec4::Vec4::new(1.0, 2.0, 3.0, 4.0);
    println!("Vec2: {:?}, Vec3: {:?}, Vec4: {:?}", v2, v3, v4);
    let m2 = mat2::Mat2::identity();
    let m3 = mat3::Mat3::identity();
    let m4 = mat4::Mat4::identity();
    println!("Mat2: {:?}, Mat3: {:?}, Mat4: {:?}", m2, m3, m4);
    let q = quat::Quat::identity();
    println!("Quat: {:?}", q);
    let t = transform::Transform::identity();
    println!("Transform: {:?}", t);
    let aabb = aabb::Aabb::new(v3, v3);
    println!("AABB: {:?}", aabb);
    let ray = ray::Ray::new(v3, v3);
    println!("Ray: {:?}", ray);
    let plane = plane::Plane::new(v3, 1.0);
    println!("Plane: {:?}", plane);
    let color = color::Color::new(1.0, 0.5, 0.25, 1.0);
    println!("Color: {:?}", color);
    let rect = rect::Rect::new(0.0, 0.0, 100.0, 50.0);
    println!("Rect: {:?}", rect);
    println!("Lerp: {}", functions::lerp(0.0, 1.0, 0.5));
    println!("Distance2: {}", functions::distance2(v2, v2));
    println!("Distance3: {}", functions::distance3(v3, v3));
}
