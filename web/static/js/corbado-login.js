(function() {
    function checkIfReady(resolve) {
        const interval = setInterval(() => {
        if (window.CORBADO_LOADED) {
            resolve(interval);
        }
    }, 100);
    }

    function ready(interval) {
        clearInterval(interval);
    }

    const waitUntilReady = new Promise(checkIfReady);
    waitUntilReady
        .then(ready)
        .then(() => {
            const authElement = document.getElementById('corbado-auth');
            Corbado.mountAuthUI(authElement, {
                onLoggedIn: () => {
                    Turbo.visit('/auth/callback/');
                },
            });
        });
})();
