use rlox_interp::lit::LitTest;

#[test]
fn simple_lit() {
    LitTest::default()
        .cmd("cargo")
        .args(vec!["run".into()])
        .checks(vec![
            "== test ==".into(),
            "0000   12 OP_CONSTANT      Number(42.0)".into(),
            "0001    | OP_CONSTANT      Number(23.0)".into(),
            "0002   13 OP_RETURN".into()])
    .test();
}
