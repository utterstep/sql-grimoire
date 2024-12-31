import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

function checkIfReady(resolve) {
    const interval = setInterval(() => {
        if (window.MONACO_LOADED) {
            resolve(interval);
        }
    }, 100);
}

function ready(interval) {
    clearInterval(interval);
}

class MonacoController extends Controller {
    static targets = ['editor'];

    connect() {
        const waitUntilReady = new Promise(checkIfReady);
        waitUntilReady
            .then(ready)
            .then(() => {
                let value = this.editorTarget.textContent;
                this.editorTarget.textContent = '';

                this.editor = monaco.editor.create(this.editorTarget, {
                    language: this.editorTarget.dataset.language,
                    theme: 'vs-dark',
                    value,
                    minimap: { enabled: false },
                });
            });
    }

    getValue() {
        return this.editor.getValue();
    }

    disconnect() {
        this.editor.dispose();
    }
}

window.application.register('monaco', MonacoController);
