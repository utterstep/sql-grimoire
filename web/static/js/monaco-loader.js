// prepend the Monaco loader
// script src="https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min/vs/loader.min.js" {}
// to the body, if it's not already there. Set the flag MONACO_LOADER_CONNECTED to true.

if (!window.MONACO_LOADER_CONNECTED) {
    const script = document.createElement('script');
    script.src = 'https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min/vs/loader.min.js';
    document.body.prepend(script);
    window.MONACO_LOADER_CONNECTED = true;
}
