import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

class EditorController extends Controller {
    static targets = ['editor'];
    static values = {
        mode: { type: String, default: 'simple' } // 'simple' or 'monaco'
    }

    connect() {
        this.tables = [];
        this.columns = [];

        if (this.modeValue === 'monaco') {
            this.initMonaco();
        } else if (this.modeValue === 'simple') {
            this.initSimple();
        } else {
            throw new Error('Invalid mode');
        }
    }

    provideCompletionItems = (model, position) => {
        // suggest read-only keywords
        const KEYWORDS = [
            'SELECT',
            'FROM',
            'WHERE',
            'GROUP BY',
            'ORDER BY',
            'LIMIT',
            'OFFSET',
            'HAVING',
            'UNION',
            'EXCEPT',
            'AS',
            'JOIN',
            'ON',
            'BETWEEN',
            'IN',
            'LIKE',
            'ILIKE',
            'IS',
            'IS NOT',
            'IS NULL',
            'IS NOT NULL',
            'AND',
            'OR',
            'NOT',
            // window functions and keywords
            'COUNT',
            'SUM',
            'AVG',
            'MIN',
            'MAX',
            'ROW_NUMBER',
            'RANK',
            'OVER',
            'PARTITION',
            'ROWS',
            'RANGE',
            'ROWS BETWEEN',
            'RANGE BETWEEN',
            'UNBOUNDED',
            'PRECEDING',
            'CURRENT ROW',
            'EXCLUDE',
            'CURRENT ROW',
            'GROUP',
            'CUME_DIST',
            'PERCENT_RANK',
        ];

        const word = model.getWordUntilPosition(position);
        const range = {
            startLineNumber: position.lineNumber,
            endLineNumber: position.lineNumber,
            startColumn: word.startColumn,
            endColumn: word.endColumn,
        };

        const suggestions = this.tables.map(table => ({
            label: table,
            insertText: table,
            kind: monaco.languages.CompletionItemKind.Class,
            range,
        })).concat(this.columns.map(column => ({
            label: column,
            insertText: column,
            kind: monaco.languages.CompletionItemKind.Field,
            range,
        }))).concat(KEYWORDS.map(keyword => ({
            label: keyword,
            insertText: keyword,
            kind: monaco.languages.CompletionItemKind.Keyword,
            range,
        })));

        return {
            suggestions,
        };
    }

    updateSchemaSuggestions({ detail: { dbInfo } }) {
        this.tables = [...new Set(dbInfo.entities.map(entity => entity.name))];
        this.columns = [...new Set(dbInfo.entities.flatMap(entity => entity.attributes.map(attribute => attribute.name)))];
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
