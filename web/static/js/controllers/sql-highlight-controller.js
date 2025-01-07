import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import { highlight } from 'https://cdn.jsdelivr.net/npm/sql-highlight@6.0.0/+esm';

function trimSqlInserts(sql) {
    // Find all INSERT INTO statements with their VALUES
    const regex = /(INSERT INTO [^;]+VALUES\s*)\(([^;]+)(\);)/gi;

    return sql.replace(regex, (match, insertPart, valuesPart, closingPart) => {
        // Get the indentation of the first value line
        const lines = valuesPart.split('\n');
        const indentMatch = lines[1] ? lines[1].match(/^\s*/) : ['  '];
        const indent = indentMatch[0];

        // Split values into individual rows
        const values = valuesPart
            .split(/,\s*(?=\()/g)
            .map(v => v.trim())
            .filter(v => v.length > 0);

        // If less than 30 values, return unchanged
        if (values.length <= 30) {
            return match;
        }

        // Get the first 15 and last 15 values
        const firstValues = values.slice(0, 15);
        const lastValues = values.slice(-15);
        const omittedCount = values.length - 30;

        // Build the new INSERT statement with proper pluralization and indentation
        const valueWord = omittedCount === 1 ? 'value' : 'values';
        const newValues = [
            firstValues[0], // First value without indentation
            ...firstValues.slice(1).map(v => `${indent}${v}`),
            `${indent}-- ${omittedCount} ${valueWord} omitted`,
            ...lastValues.map(v => `${indent}${v}`)
        ].join(',\n');

        return `${insertPart}(${newValues}${closingPart}`;
    });
}

class SqlHighlightController extends Controller {
    static targets = ['code'];

    connect() {
        this.highlight()
    }

    highlight() {
        this.codeTargets.forEach(node => {
            const sqlContent = trimSqlInserts(node.textContent);
            const highlighted = highlight(sqlContent, {
                html: true
            });

            node.innerHTML = highlighted;
        });
    }
}

window.application.register('sql-highlight', SqlHighlightController);
