import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

class SchemaHiddenController extends Controller {
    static targets = ['schemaSelector', 'schemaHidden'];

    async querySchema(id) {
        const response = await fetch(`/admin/exercise/schemas/${id}/json/`);
        const schema = await response.json();

        this.schemaHiddenTarget.textContent = schema.schema;
    }

    async connect() {
        // if something is selected, query the schema
        if (this.schemaSelectorTarget.value) {
            await this.querySchema(this.schemaSelectorTarget.value);
        }
    }

    async processChange() {
        const id = this.schemaSelectorTarget.value;

        if (id) {
            await this.querySchema(id);
        }
    }
}

window.application.register('schema-hidden', SchemaHiddenController);
