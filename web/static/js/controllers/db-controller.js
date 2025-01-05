import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import { PGlite } from 'https://cdn.jsdelivr.net/npm/@electric-sql/pglite/dist/index.js';

export default class DbController extends Controller {
    static targets = ['schema'];

    async connect() {
        const schemaCreationQueries = this.hasSchemaTarget ? this.schemaTarget.textContent : null;

        if (schemaCreationQueries) {
            await this.resetDb(schemaCreationQueries);
        }
    }

    async schemaUpdated({ detail: { schema: { schema } } }) {
        await this.resetDb(schema);
    }

    async resetDb(schema) {
        this.db = await PGlite.create();

        await this.db.exec(schema);
    }

    async runQuery(query) {
        if (!this.db) {
            throw new Error('Database not initialized');
        }

        return await this.db.query(query, [], { rowMode: 'array' });
    }
}

window.application.register('db', DbController);
