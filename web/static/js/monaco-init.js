(function() {
    function checkIfReady(resolve) {
        const interval = setInterval(() => {
            if (window.MONACO_LOADER_CONNECTED) {
                resolve(interval);
            }
        }, 50);
    }

    function ready(interval) {
        clearInterval(interval);
    }

    const waitUntilReady = new Promise(checkIfReady);
    waitUntilReady
        .then(ready)
        .then(() => {
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
        });
})();
