import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

class SchemaHiddenController extends Controller {
    static targets = ['schemaSelector', 'schemaHidden'];

    async connect() {
        // if something is selected, query the schema
        if (this.schemaSelectorTarget.value) {
            await this.fetchSchema();
        }
    }

    updateSchema(schema) {
        this.schemaHiddenTarget.textContent = schema;

        this.dispatch('schema-updated', { detail: { schema } });
    }

    async fetchSchema() {
        const id = this.schemaSelectorTarget.value;

        if (id) {
            const response = await fetch(`/admin/exercise/schemas/${id}/json/`);
            const schema = await response.json();

            this.updateSchema(schema);
        }
    }
}

window.application.register('schema-hidden', SchemaHiddenController);
