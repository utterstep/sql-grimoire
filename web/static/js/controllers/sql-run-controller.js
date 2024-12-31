import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import { PGlite } from 'https://cdn.jsdelivr.net/npm/@electric-sql/pglite/dist/index.js';

class SqlRunController extends Controller {
    static targets = ['schema', 'results'];
    static outlets = ['simple-editor', 'monaco'];

    async connect() {
        this.db = await PGlite.create();

        const schemaCreationQueries = this.schemaTarget.textContent;
        await this.db.exec(schemaCreationQueries);
    }

    async runQuery(query) {
        const result = await this.db.query(query, [], { rowMode: 'array' });

        return result;
    }

    getQuery() {
        if (this.hasMonacoOutlet) {
            return this.monacoOutlet.getValue();
        } else if (this.hasSimpleEditorOutlet) {
            return this.simpleEditorOutlet.getValue();
        } else {
            throw new Error('No editor outlet found');
        }
    }

    async execute() {
        const query = this.getQuery();

        const result = await this.runQuery(query);

        console.log('result', result);

        if (!result.fields || !result.fields.length) {
            this.resultsTarget.innerHTML = 'No results';

            console.warn('Unexpected result', result);

            return;
        }

        let html = `
            <table class="table">
                <thead>
                    <tr>
                        ${result.fields.map(field => `<th class="table__header">${field.name}</th>`).join('')}
                    </tr>
                </thead>
                <tbody>
                    ${result.rows.map(row => `
                        <tr>
                            ${row.map(value => `<td class="table__cell">${value}</td>`).join('')}
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;

        this.resultsTarget.innerHTML = html;
        this.resultsTarget.scrollIntoView({ behavior: 'smooth' });
    }

    async executeToTextArea() {
        const query = this.getQuery();
        const result = await this.runQuery(query);

        // reformat result to be a array of objects
        const formattedResult = result.rows.map((row) => {
            return Object.fromEntries(
                row.map((value, index) => [result.fields[index].name, value]),
            );
        });

        this.resultsTarget.value = JSON.stringify(formattedResult, null, 2);
    }
}

window.application.register('sql-run', SqlRunController);
