use static_file_util::static_files;

static_files!(
    (main_css, "../static/main.css", mime::TEXT_CSS),
    (
        corbado_init,
        "../static/js/corbado-init.js",
        mime::TEXT_JAVASCRIPT
    ),
    (
        corbado_login,
        "../static/js/corbado-login.js",
        mime::TEXT_JAVASCRIPT
    ),
    (lucide, "../static/js/lucide.js", mime::TEXT_JAVASCRIPT),
    (
        monaco_init,
        "../static/js/monaco-init.js",
        mime::TEXT_JAVASCRIPT
    ),
    (
        stimulus_init,
        "../static/js/stimulus-init.js",
        mime::TEXT_JAVASCRIPT
    ),
    // controllers
    (
        highlight_controller,
        "../static/js/controllers/highlight-controller.js",
        mime::TEXT_JAVASCRIPT
    ),
    (
        monaco_controller,
        "../static/js/controllers/monaco-controller.js",
        mime::TEXT_JAVASCRIPT
    ),
    (
        schema_hidden_controller,
        "../static/js/controllers/schema-hidden-controller.js",
        mime::TEXT_JAVASCRIPT
    ),
    (
        simple_editor_controller,
        "../static/js/controllers/simple-editor.js",
        mime::TEXT_JAVASCRIPT
    ),
    (
        solution_submit_controller,
        "../static/js/controllers/solution-submit-controller.js",
        mime::TEXT_JAVASCRIPT
    ),
    (
        sql_run_controller,
        "../static/js/controllers/sql-run-controller.js",
        mime::TEXT_JAVASCRIPT
    ),
);
