import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

import hljs from 'https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/highlight.min.js';
import pgsql from 'https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/languages/pgsql.min.js';

hljs.registerLanguage('pgsql', pgsql);

class HighlightController extends Controller {
    static targets = ['code'];

    connect() {
        this.highlight()
    }

    highlight() {
        this.codeTargets.forEach(element => {
            if (!element.dataset.highlighted) {
                hljs.highlightElement(element);
            }
        });
    }
}

window.application.register('highlight', HighlightController);
