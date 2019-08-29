use rlox_interp::lit::LitTest;

#[test]
fn simple_lit() {
    LitTest {
        cmd: "cargo".into(),
        args: vec!["run".into()],
        checks: vec![],
    }
    .test();
}
