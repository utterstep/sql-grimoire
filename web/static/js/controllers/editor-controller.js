import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

import { Compartment } from "@codemirror/state";
import { basicSetup, EditorView } from "https://esm.sh/codemirror@6.0.1?external=@codemirror/state";
import { sql, PostgreSQL } from "https://esm.sh/@codemirror/lang-sql@6.8.0?external=@codemirror/state";

import { vscodeDark } from 'https://esm.sh/@uiw/codemirror-theme-vscode?external=@codemirror/state';

class EditorController extends Controller {
    static targets = ['editor'];
    static values = {
        mode: { type: String, default: 'simple' } // 'simple' or 'monaco'
    }

    connect() {
        if (this.modeValue === 'codemirror') {
            this.initCodemirror();
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

    initCodemirror() {
        function htmlDecode(input) {
            const doc = new DOMParser().parseFromString(input, "text/html");
            return doc.documentElement.textContent;
        }

        const sqlCompartment = new Compartment();
        let sqlExtension = sql({ dialect: PostgreSQL, upperCaseKeywords: true });

        const view = new EditorView({
            extensions: [basicSetup, vscodeDark, sqlCompartment.of(sqlExtension)],
            parent: this.editorTarget,
        });

        const transaction = view.state.update({
            changes: { from: 0, insert: htmlDecode(this.editorTarget.textContent) },
        });
        view.dispatch(transaction);

        // include schema
        sqlExtension = sql({
            dialect: PostgreSQL,
            upperCaseKeywords: true,
            defaultSchema: "public",
            schema: {
            "public": {
                "manufacturers": [
                    "code",
                    "name"
                ],
                "products": [
                    "code",
                    "name",
                    "price",
                    "manufacturer"
                ]
        }}});

        view.dispatch({
            effects: sqlCompartment.reconfigure(sqlExtension),
        });

        this.editor = view;
    }

    getValue() {
        if (this.modeValue === 'codemirror') {
            return this.editor.state.doc.toString();
        }
        return this.editorTarget.value;
    }

    disconnect() {
        if (this.modeValue === 'codemirror' && this.editor) {
            this.editor.destroy();
        }
    }
}

window.application.register('editor', EditorController);
