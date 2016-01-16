use rustc_serialize::json::Json;
use std::fs::File;
use std::io::prelude::*;
use term::*;


macro_rules! parse_i32 {
    ($json: expr, $key: expr, $default: expr) => {
        {
            match $json.find($key) {
                Some(x) => x.as_i64().unwrap() as i32,
                None => $default,
            }
        }
    };
}


macro_rules! parse_string {
    ($json: expr, $key: expr, $default: expr) => {
        {
            match $json.find($key) {
                Some(x) => x.as_string().unwrap().to_string(),
                None => $default as String,
            }
        }
    };
}


/// Load a config from file
pub fn load_config_file(path: String) -> Canvas {
    let mut buffer = String::new();
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut buffer).unwrap();
    load_config(buffer)
}


/// Load a JSON string to construct window tree
pub fn load_config(config: String) -> Canvas {

    // println!("{}", config);
    let data = match Json::from_str(&*config) {
        Ok(d) => d,
        Err(e) => panic!("Unable to read json: {:?}", e),
    };
    let mut canvas = Canvas::new();
    canvas.initialize();

    canvas.delay = parse_i32!(data, "delay", 1000);

    for json_window in data["root"].as_array().unwrap() {
        let win = load_window_from_json_object(json_window);
        win.draw();
        canvas.root.children.push(win);
    }

    canvas
}


fn load_window_from_json_object(json: &Json) -> Window {
    let x_offset = parse_i32!(json, "x_offset", 0);
    let y_offset = parse_i32!(json, "y_offset", 0);
    let width = parse_i32!(json, "width", 0);
    let height = parse_i32!(json, "height", 0);
    let border_v = parse_i32!(json, "border-vertical", 0) as u64;
    let border_h = parse_i32!(json, "border-horizontal", 0) as u64;

    assert!(json["content"].as_object().is_some(),
            "Missing window content!");

    let source_val = parse_string!(json["content"], "source", "".to_string());
    let source_code = parse_string!(json["content"], "type", "StaticText".to_string());
    let source = match &*source_code {
        "Subcommand" => Source::Subcommand(source_val),
        _ => Source::StaticText(source_val),
    };

    let content_code = parse_string!(json, "type", "".to_string());
    let content = match &*content_code {
        "Text" => Content::Text(source),
        "BigText" => Content::BigText(source),
        _ => Content::Text(source),
    };

    let mut win = Window::new(x_offset, y_offset, width, height, content);
    win.border_h = border_h;
    win.border_v = border_v;
    win

}
