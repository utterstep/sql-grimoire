import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

// Resizes an element by dragging its handle.
//
// Resizes only horizontally. Reacts to touch and mouse events.
class DragResizeController extends Controller {
    static targets = ['handle', 'resizable'];

    connect() {
        const handle = this.handleTarget;

        handle.addEventListener('mousedown', this.startDrag.bind(this));
        handle.addEventListener('touchstart', this.startDrag.bind(this));
    }

    resetWidth() {
        if (this.initialWidth) {
            this.resizableTarget.style.width = `${this.initialWidth}px`;
        }
    }

    startDrag(event) {
        event.preventDefault();

        const resizable = this.resizableTarget;

        const startX = event.clientX ?? event.touches[0].clientX;
        const startWidth = resizable.offsetWidth;

        if (!this.initialWidth) {
            this.initialWidth = startWidth;
        }

        const move = (event) => {
            const clientX = event.clientX ?? event.touches[0].clientX;
            const dx = -(clientX - startX);
            resizable.style.width = `${startWidth + dx}px`;
        };

        const end = () => {
            document.removeEventListener('mousemove', move);
            document.removeEventListener('mouseup', end);
            document.removeEventListener('touchmove', move);
            document.removeEventListener('touchend', end);
        };

        document.addEventListener('mousemove', move);
        document.addEventListener('mouseup', end);
        document.addEventListener('touchmove', move, { passive: true });
        document.addEventListener('touchend', end);
    }
}

window.application.register('drag-resize', DragResizeController);
