import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

class EditorController extends Controller {
    static targets = ['editor'];
    static values = {
        mode: { type: String, default: 'simple' } // 'simple' or 'monaco'
    }

    connect() {
        if (this.modeValue === 'monaco') {
            this.initMonaco();
        } else if (this.modeValue === 'simple') {
            this.initSimple();
        } else {
            throw new Error('Invalid mode');
        }
    }

    initSimple() {
        // For simple mode, we don't need to do anything special
        // The textarea is ready to use as-is
    }

    initMonaco() {
        function checkIfReady(resolve) {
            const interval = setInterval(() => {
                if (window.MONACO_LOADED) {
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
        if (this.modeValue === 'monaco') {
            return this.editor.getValue();
        }
        return this.editorTarget.value;
    }

    disconnect() {
        if (this.modeValue === 'monaco' && this.editor) {
            this.editor.dispose();
        }
    }
}

window.application.register('editor', EditorController);
