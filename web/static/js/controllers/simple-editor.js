import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

class SimpleEditorController extends Controller {
    static targets = ['editor'];

    connect() {
        console.log('SimpleEditorController connected');
    }

    getValue() {
        return this.editorTarget.value;
    }
}

window.application.register('simple-editor', SimpleEditorController);
