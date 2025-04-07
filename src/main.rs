slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();
    let win = ui.as_weak().unwrap();
    ui.on_button_clicked(move |text| {
        println!("User: {}, Pass: {}", win.get_username(), win.get_password());
        win.set_response(format!("{} coming.", win.get_username()).into());
        format!("Hello {}", text).into()
    });
    ui.run().unwrap();
}
