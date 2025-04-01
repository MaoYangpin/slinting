slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();
    ui.on_button_clicked(move || "Hello".into());
    ui.run().unwrap();
}
