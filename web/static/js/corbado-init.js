function getCookieByName(name) {
    const cookies = document.cookie.split(';');
    for (let cookie of cookies) {
         cookie = cookie.trim();
         if (cookie.startsWith(name + '=')) {
            return cookie.substring(name.length + 1);
         }
    }
   return null;
}

if (!window.CORBADO_LOADED) {
    const projectId = getCookieByName('corbado_project_id');

    await Corbado.load({
        darkMode: 'on',
        projectId,
    });

    window.CORBADO_LOADED = true;
}
