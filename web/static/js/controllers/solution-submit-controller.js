import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';

class SolutionSubmitController extends Controller {
    static outlets = ['sql-run', 'monaco'];

    async submit() {
        const query = this.monacoOutlet.getValue();
        const result = await this.sqlRunOutlet.runQuery(query);

        // reformat result to be a array of objects
        const formattedResult = result.rows.map((row) => {
            return Object.fromEntries(
                row.map((value, index) => [result.fields[index].name, value]),
            );
        });

        // post to current url + submit with json { query, result }
        const url = new URL(window.location.href);
        url.pathname += 'submit/';

        fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ query, result: formattedResult }),
        })
        .then(() => {
            Turbo.visit(window.location.href);
        })
        .catch((error) => {
            console.error('Error submitting solution:', error);
        });
    }
}

window.application.register('solution-submit', SolutionSubmitController);
