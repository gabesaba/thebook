use testing;

mod gabes_module;

#[test]
fn it_multiplies_gud() {
    assert_eq!(gabes_module::get_ans(), testing::multiply(21, 2));
    assert_eq!(gabes_module::get_ans(), testing::multiply(42, 1));
}
