use my_macro::show_streams;

#[show_streams(a = "123", b = 666)]
pub fn invoke2(
    #[query_arg(name = "1")]
    #[check(max_len = 64, min_len = 32)]
    _a: &str,
    #[json_arg(name = "1")]
    #[many_props(a = "hi, master!", b = 2, c = true, d = 6.66, e = ';')]
    _b: &str
) {

}

