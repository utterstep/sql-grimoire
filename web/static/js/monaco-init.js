if (!window.MONACO_LOADED) {
    require.config({
        paths: {
            vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min/vs',
        },
    });

    require(['vs/editor/editor.main'], function() {
        window.MONACO_LOADED = true;
    });
}
