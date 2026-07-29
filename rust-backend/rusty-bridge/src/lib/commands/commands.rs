use serde_json::json;

use crate::{
    command, command_struct,
    user_events::{ResizeDirection, UserEvent, WindowEvent},
    utils::{get_field_as_bool, get_field_as_string, put_value_in_result},
    webview_windows_manager::WebViewWindowConfig,
};

command!(log, |a, _g| {
    let name = get_field_as_string(&a, "name");
    put_value_in_result(&json!({"name": name}))
});

command_struct!(minimize_window, |a, g| {
    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::Minimize(true),
    ));
    Ok(String::new())
});

command_struct!(move_window, |a, g| {
    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::DragWindow,
    ));
    Ok(String::from("Moving window"))
});

command_struct!(set_decoration, |a, g| {
    let decoration = get_field_as_bool(&a, "decoration");
    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::HideDecoration(decoration),
    ));
    Ok(String::from("Moving window"))
});

command_struct!(create_window, |a, g| {
    let window_config: WebViewWindowConfig =
        serde_json::from_value(a.clone()).map_err(|e| e.to_string())?;

    g.event_loop_proxy
        .send_event(UserEvent::CreateNewWindow(window_config))
        .map_err(|a| a.to_string())?;

    Ok(String::from("Created"))
});

command_struct!(resize_window, |a, g| {
    let direction = get_field_as_string(&a, "direction");

    let direction = match direction.as_str() {
        "Left" => ResizeDirection::Left,
        "Right" => ResizeDirection::Right,
        "Top" => ResizeDirection::Top,
        "Bottom" => ResizeDirection::Bottom,
        "TopLeft" => ResizeDirection::TopLeft,
        "TopRight" => ResizeDirection::TopRight,
        "BottomLeft" => ResizeDirection::BottomLeft,
        "BottomRight" => ResizeDirection::BottomRight,
        _ => ResizeDirection::None,
    };

    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::ResizeWindow(direction),
    ));

    Ok(String::from("Resizing window"))
});

command_struct!(close_window, |a, g| {
    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::CloseWindow,
    ));
    Ok(String::new())
});
