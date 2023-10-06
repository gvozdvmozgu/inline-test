use std::path::Path;

fn main() {
    let tests = Path::new("./test_data");

    inline_test::for_each(|name, text| {
        let test_file = tests.join(name);
        std::fs::write(test_file, text).unwrap();
    });
}
