import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import { highlight } from 'https://cdn.jsdelivr.net/npm/sql-highlight@6.0.0/+esm';

class HighlightController extends Controller {
    static targets = ['code'];

    connect() {
        this.highlight()
    }

    highlight() {
        this.codeTargets.forEach(node => {
            const sqlContent = node.textContent;
            const highlighted = highlight(sqlContent, {
                html: true
            });

            node.innerHTML = highlighted;
        });
    }
}

window.application.register('highlight', HighlightController);
