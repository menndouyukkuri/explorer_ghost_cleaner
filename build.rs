use winres::WindowsResource;

fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut res = WindowsResource::new();
        res.set_icon_with_id("icon/icon.ico", "1");
        res.compile().expect("Error occurred when embedding an icon.");
    }
}
