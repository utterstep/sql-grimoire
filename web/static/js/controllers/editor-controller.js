import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import { SQLAutocomplete, SQLDialect } from 'https://esm.sh/sql-autocomplete@1.1.1?bundle-deps';

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

    provideCompletionItems(model, position) {
        const word = model.getWordUntilPosition(position);
        const range = {
            startLineNumber: position.lineNumber,
            endLineNumber: position.lineNumber,
            startColumn: word.startColumn,
            endColumn: word.endColumn,
        };

        const line = model.getLineContent(position.lineNumber);
        const index = position.column;

        const suggestions = this.sqlAutocomplete.autocomplete(line, index);

        const optionTypeMap = {
            'TABLE': monaco.languages.CompletionItemKind.Class,
            'COLUMN': monaco.languages.CompletionItemKind.Field,
            'KEYWORD': monaco.languages.CompletionItemKind.Keyword,
        }

        return {
            suggestions: suggestions.filter(suggestion => !!suggestion.value).map(suggestion => ({
                label: suggestion.value,
                insertText: suggestion.value,
                kind: optionTypeMap[suggestion.optionType],
                range,
            })),
        }
    }

    initSchemaSuggestions({ detail: { dbInfo } }) {
        this.tables = [...new Set(dbInfo.entities.map(entity => entity.name))];
        this.columns = [...new Set(dbInfo.entities.flatMap(entity => entity.attributes.map(attribute => attribute.name)))];

        this.sqlAutocomplete = new SQLAutocomplete(SQLDialect.PLpgSQL, this.tables, this.columns);
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

                if (!this.sqlAutocomplete) {
                    this.sqlAutocomplete = new SQLAutocomplete(SQLDialect.PLpgSQL);
                }

                monaco.languages.registerCompletionItemProvider("pgsql", {
                    provideCompletionItems: this.provideCompletionItems.bind(this),
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
