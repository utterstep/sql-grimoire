import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

class SchemaHiddenController extends Controller {
    static targets = ['schemaSelector', 'schemaHidden'];

    async updateSchema(id) {
        const response = await fetch(`/admin/exercise/schemas/${id}/json/`);
        const schema = await response.json();

        this.dispatch('schema-updated', { detail: { schema } });

        this.schemaHiddenTarget.textContent = schema.schema;
    }

    async connect() {
        // if something is selected, query the schema
        if (this.schemaSelectorTarget.value) {
            await this.updateSchema(this.schemaSelectorTarget.value);
        }
    }

    async processChange() {
        const id = this.schemaSelectorTarget.value;

        if (id) {
            await this.updateSchema(id);
        }
    }
}

window.application.register('schema-hidden', SchemaHiddenController);
