import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import { PGlite } from 'https://cdn.jsdelivr.net/npm/@electric-sql/pglite/dist/index.js';

export default class DbController extends Controller {
    static targets = ['schema'];

    async connect() {
        const schemaCreationQueries = this.getSchema();

        if (schemaCreationQueries) {
            await this.resetDb(schemaCreationQueries);
        }
    }

    getSchema() {
        // check the value first (bc textarea has both value and textContent)
        return this.hasSchemaTarget && (this.schemaTarget.value || this.schemaTarget.textContent);
    }

    async schemaUpdated({ detail: { schema: { schema } } }) {
        await this.resetDb(schema);
    }

    async resetDbRequest(e) {
        const schema = this.getSchema();

        if (!schema) {
            return;
        }

        await this.resetDb(schema);
    }

    async resetDb(schema) {
        if (this.db) {
            await this.db.close();
        }

        this.db = await PGlite.create();

        await this.db.exec(schema);

        this.dispatch('db-created');
    }

    async runQuery(query) {
        if (!this.db) {
            throw new Error('Database not initialized');
        }

        return await this.db.query(query, [], { rowMode: 'array' });
    }

    async disconnect() {
        if (this.db) {
            await this.db.close();
        }
    }
}

window.application.register('db', DbController);
