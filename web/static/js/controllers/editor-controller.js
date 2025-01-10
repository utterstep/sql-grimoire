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

    initSchemaSuggestions({ detail: { dbInfo } }) {
        const tables = [...new Set(dbInfo.entities.map(entity => entity.name))];
        const columns = [...new Set(dbInfo.entities.flatMap(entity => entity.attributes.map(attribute => attribute.name)))];

        this.createFieldsProposals = (range) => {
            return tables.map(table => ({
                label: table,
                insertText: table,
                kind: monaco.languages.CompletionItemKind.Class,
                range,
            })).concat(columns.map(column => ({
                label: column,
                insertText: column,
                kind: monaco.languages.CompletionItemKind.Field,
                range,
            })));
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
                const value = this.editorTarget.textContent;
                this.editorTarget.textContent = '';

                const provideCompletionItems = (model, position) => {
                    const word = model.getWordUntilPosition(position);
                    const range = {
                        startLineNumber: position.lineNumber,
                        endLineNumber: position.lineNumber,
                        startColumn: word.startColumn,
                        endColumn: word.endColumn,
                    };

                    return {
                        suggestions: this.createFieldsProposals(range),
                    };
                }

                monaco.languages.registerCompletionItemProvider("pgsql", {
                    provideCompletionItems: provideCompletionItems.bind(this),
                });

                this.editor = monaco.editor.create(this.editorTarget, {
                    language: this.editorTarget.dataset.language,
                    theme: 'vs-dark',
                    fontSize: 14,
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
