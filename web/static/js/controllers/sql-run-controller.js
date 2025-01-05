import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

class SqlRunController extends Controller {
    static targets = ['results'];
    static outlets = ['editor', 'db'];

    async execute(e) {
        e.preventDefault();

        const query = this.editorOutlet.getValue();

        let html;

        try {
            const result = await this.dbOutlet.runQuery(query);

            console.log('result', result);

            if (!result.fields || !result.fields.length) {
                this.resultsTarget.innerHTML = 'No results';

                console.warn('Unexpected result', result);

                return;
            }

            html = `
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
        } catch (e) {
            console.error(e);

            const targetClass = `${this.resultsTarget.classList[0]}__error`;

            html = `
                <div class="${targetClass}">
                    <p>Error: ${e.message}</p>
                </div>
            `;
        }

        this.resultsTarget.innerHTML = html;
        this.resultsTarget.scrollIntoView({ behavior: 'smooth' });
    }

    async executeToTextArea(e) {
        e.preventDefault();

        const query = this.editorOutlet.getValue();
        const result = await this.dbOutlet.runQuery(query);

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
