#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|code: &str| {
    if let Ok(ast) = full_moon::parse(code) {
        let printed = full_moon::print(&ast);
        assert_eq!(code, printed);
    }
});
