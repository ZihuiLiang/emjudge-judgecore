#[test]
fn language_info() {
    let compile_and_exe_settings =
        emjudge_judgecore::settings::CompileAndExeSettings::load_from_file(
            "examples/compile_and_exe_settings.toml",
            config::FileFormat::Toml,
        )
        .unwrap();
    for (language, setting) in compile_and_exe_settings.get_languages_info().unwrap() {
        println!("{}:\n{}", language, setting);
    }
}
