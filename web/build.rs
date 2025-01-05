use static_file_util::process_file;

fn main() {
    process_file("static/main.css", "main_css_HASH");
    process_file("static/reset.css", "reset_css_HASH");

    process_file("static/js/corbado-init.js", "corbado_init_HASH");
    process_file("static/js/corbado-login.js", "corbado_login_HASH");
    process_file("static/js/lucide.js", "lucide_HASH");
    process_file("static/js/monaco-init.js", "monaco_init_HASH");
    process_file("static/js/monaco-loader.js", "monaco_loader_HASH");
    process_file("static/js/stimulus-init.js", "stimulus_init_HASH");

    // controllers
    process_file(
        "static/js/controllers/highlight-controller.js",
        "highlight_controller_HASH",
    );
    process_file(
        "static/js/controllers/monaco-controller.js",
        "monaco_controller_HASH",
    );
    process_file(
        "static/js/controllers/schema-hidden-controller.js",
        "schema_hidden_controller_HASH",
    );
    process_file(
        "static/js/controllers/solution-submit-controller.js",
        "solution_submit_controller_HASH",
    );
    process_file(
        "static/js/controllers/simple-editor.js",
        "simple_editor_controller_HASH",
    );
    process_file(
        "static/js/controllers/sql-run-controller.js",
        "sql_run_controller_HASH",
    );
}
