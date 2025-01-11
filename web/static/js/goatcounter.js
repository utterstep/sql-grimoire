window.goatcounter = {no_onload: true};

document.documentElement.addEventListener('turbo:load', function(e) {
    if (window.goatcounter.count) {
        window.goatcounter.count({
            path: location.pathname + location.search + location.hash,
        });
    }
});
