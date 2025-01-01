import { Application } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

if (!window.application) {
    window.application = new Application();
    window.application.start();
}
