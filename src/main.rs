slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();
    ui.on_button_clicked(move |text| format!("Hello {}", text).into());
    ui.run().unwrap();
}
