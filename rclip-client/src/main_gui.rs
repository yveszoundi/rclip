#![windows_subsystem = "windows"]

use fltk::{app, button, dialog, group, input, prelude::*, window};
use rclip_config;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

mod common;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);

    let wind_title = format!(
        "{} {}",
        option_env!("CARGO_PKG_NAME").unwrap_or("Unknown"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("Unknown")
    );

    let mut wind = window::Window::default()
        .with_size(430, 230)
        .center_screen()
        .with_label(&wind_title);
    let wind_copy = wind.clone();
    let wind_ref_receive = wind.clone();
    let wind_ref_clear = wind.clone();
    wind.make_resizable(true);

    let size_pack_spacing = 10;

    let mut group_host = group::Pack::default()
        .with_pos(100, 20)
        .with_size(400, 40)
        .with_type(group::PackType::Horizontal);
    group_host.set_spacing(size_pack_spacing);
    let input_host = Rc::new(RefCell::new(
        input::Input::default()
            .with_size(200, 20)
            .with_label("Server host"),
    ));

    let client_config =
        match rclip_config::load_default_config(common::DEFAULT_CONFIG_FILENAME_CLIENT) {
            Ok(cfg) => cfg,
            _ => rclip_config::ClientConfig::default(),
        };

    let input_host_copy = input_host.clone();
    input_host.borrow_mut().set_tooltip("IP address to bind to");
    if let Some(server_host) = client_config.server.host {
        input_host.borrow_mut().set_value(&server_host);
    }

    group_host.end();

    let mut group_port = group::Pack::default()
        .below_of(&group_host, size_pack_spacing)
        .with_size(400, 40)
        .with_type(group::PackType::Horizontal);
    group_port.set_spacing(size_pack_spacing);
    let input_port = Rc::new(RefCell::new(
        input::Input::default()
            .with_size(200, 20)
            .with_label("Server port"),
    ));
    let input_port_copy = input_port.clone();

    if let Some(server_port) = client_config.server.port {
        let port_number_text = format!("{}", server_port);

        input_port.borrow_mut().set_value(&port_number_text);
    }

    input_port.borrow_mut().set_tooltip("Server port number");
    group_port.end();

    let mut group_pub_cert = group::Pack::default()
        .below_of(&group_port, size_pack_spacing)
        .with_size(400, 40)
        .with_type(group::PackType::Horizontal);
    group_pub_cert.set_spacing(size_pack_spacing);
    let input_pub_cert = Rc::new(RefCell::new(
        input::Input::default()
            .with_size(200, 20)
            .with_label("Public key"),
    ));

    if let Some(pub_key_loc) =
        rclip_config::resolve_default_cert_path(rclip_config::DEFAULT_FILENAME_DER_CERT_PUB)
    {
        input_pub_cert.borrow_mut().set_value(&pub_key_loc);
    }

    let input_pub_cert_copy = input_pub_cert.clone();
    let input_pub_cert_copy2 = input_pub_cert.clone();
    let input_pub_cert_copy3 = input_pub_cert.clone();
    input_pub_cert
        .borrow_mut()
        .set_tooltip("Public DER key path");
    let mut button_pub_cert = button::Button::default()
        .with_size(80, 20)
        .with_label("Browse...");
    button_pub_cert.set_callback({
        move |_| {
            let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
            dlg.set_title("Select public key");
            dlg.show();

            let selected_filename = dlg.filename();

            if !selected_filename.as_os_str().is_empty() {
                let path_name = format!("{}", dlg.filename().display());
                input_pub_cert.borrow_mut().set_value(&path_name);
            }
        }
    });
    group_pub_cert.end();

    let mut group_buttons = group::Pack::default()
        .with_size(400, 40)
        .below_of(&group_pub_cert, size_pack_spacing)
        .with_type(group::PackType::Horizontal);
    group_buttons.set_spacing(size_pack_spacing);

    let mut button_receive = button::Button::default()
        .with_size(80, 20)
        .with_label("Read");
    let mut button_send = button::Button::default()
        .with_size(80, 20)
        .with_label("Write");
    let mut button_clear = button::Button::default()
        .with_size(80, 20)
        .with_label("Clear");

    fn send_cmd(
        host_text: String,
        port_text: String,
        key_pub_der: String,
        cmd_name: &str,
        cmd_text: Option<String>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let clipboard_cmd = common::ClipboardCmd {
            name: cmd_name.to_string(),
            text: cmd_text,
        };

        let server_port = port_text.parse::<u16>()?;

        match common::send_cmd(host_text, server_port, key_pub_der, clipboard_cmd) {
            Ok(_) => Ok(()),
            Err(ex) => Err(format!("{}", ex.to_string()).into()),
        }
    }

    button_send.set_callback({
        let c_input_host = input_host.clone();
        let c_input_port = input_port.clone();

        move |_| {
            let port_text = c_input_port.borrow().value();
            let host_text = c_input_host.borrow().value();
            let cert_path = input_pub_cert_copy2.borrow().value();

            if let Ok(clipboard_contents) = common::get_clipboard_contents() {
                let cmd_text_opt = Some(clipboard_contents);
                let wind_copy = wind_copy.clone();

                if let Err(ex) = send_cmd(host_text, port_text, cert_path, "WRITE", cmd_text_opt) {
                    dialog::alert(
                        wind_copy.x(),
                        wind_copy.y() + wind_copy.height() / 2,
                        ex.to_string().as_str(),
                    );
                }
            } else {
                dialog::alert(
                    wind_copy.x(),
                    wind_copy.y() + wind_copy.height() / 2,
                    "Could not acquire clipboard contents!",
                );
            }
        }
    });

    button_clear.set_callback({
        let c_input_host = input_host.clone();
        let c_input_port = input_port.clone();

        move |_| {
            let port_text = c_input_port.borrow().value();
            let host_text = c_input_host.borrow().value();
            let cmd_text_opt = Some(String::new());
            let cert_path = input_pub_cert_copy3.borrow().value();
            let wind_ref_clear = wind_ref_clear.clone();

            if let Err(ex) = send_cmd(host_text, port_text, cert_path, "CLEAR", cmd_text_opt) {
                dialog::alert(
                    wind_ref_clear.x(),
                    wind_ref_clear.y() + wind_ref_clear.height() / 2,
                    ex.to_string().as_str(),
                );
            }
        }
    });

    button_receive.set_callback({
        move |_| {
            let port_text = input_port_copy.borrow().value();
            let host_text = input_host_copy.borrow().value();
            let cert_path = input_pub_cert_copy.borrow().value();
            let wind_ref_receive = wind_ref_receive.clone();

            if let Err(ex) = send_cmd(host_text, port_text, cert_path, "READ", None) {
                dialog::alert(
                    wind_ref_receive.x(),
                    wind_ref_receive.y() + wind_ref_receive.height() / 2,
                    ex.to_string().as_str(),
                );
            }
        }
    });

    group_buttons.end();

    wind.end();
    wind.show();

    match app.run() {
        Ok(_) => Ok(()),
        Err(ex) => Err(ex.into()),
    }
}
