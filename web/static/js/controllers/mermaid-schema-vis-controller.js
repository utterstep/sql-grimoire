import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
import Panzoom from 'https://cdn.jsdelivr.net/npm/@panzoom/panzoom@4.5.1/+esm';

mermaid.initialize({
    startOnLoad: false,
    theme: 'dark',
});

class MermaidSchemaVisController extends Controller {
    static targets = ['schemaVis'];

    generateEntity(entity) {
        const mermaid = [`    ${entity.name} {`];

        for (const attribute of entity.attributes) {
            const { comment, key, name, type } = attribute;
            mermaid.push(
                `        ${name} ${type}${key ? ` ${key} ` : ' '}"${comment}"`
            );
        }

        mermaid.push('    }');

        return mermaid.join('\n');
    }

    generateEntities(entities) {
        const mermaid = [];

        for (const entity of entities) {
            mermaid.push(this.generateEntity(entity));
        }

        return mermaid.join('\n\n');
    }

    generateRelationships(relationships) {
        const mermaid = [];

        for (const { child, parent } of relationships) {
            mermaid.push(
                `    ${parent.entity} ||--o{ ${child.entity} : "${child.entity
                }(${child.attributes.join(', ')}) -> ${parent.entity
                }(${parent.attributes.join(', ')})"`
            );
        }

        return mermaid.join('\n');
    }

    generateDiagram({ entities = [], relationships = [] }) {
        const diagram = ['erDiagram'];

        if (entities.length > 0) {
            diagram.push(this.generateEntities(entities));
        }

        if (relationships.length > 0) {
            diagram.push(this.generateRelationships(relationships));
        }

        return diagram.join('\n\n');
    }

    async drawSchema({ detail: { dbInfo } }) {
        const {
            entities,
            relationships,
            indexes: _indexes,
        } = dbInfo;
        const diagram = this.generateDiagram({ entities, relationships });

        // insert a single child element into the schemaVisTarget
        // child should be a pre with the class mermaid and content the diagram
        const pre = document.createElement('pre');
        pre.classList.add('mermaid');
        pre.textContent = diagram;
        this.schemaVisTarget.replaceChildren(pre);

        // run mermaid
        mermaid.run({
            nodes: Array.from(this.schemaVisTarget.children),
        });

        this.panzoom = Panzoom(this.schemaVisTarget, {
            minScale: 1,
            maxScale: 3,
            step: 0.1,
        });
        this.schemaVisTarget.parentElement.addEventListener(
            'wheel',
            this.panzoom.zoomWithWheel,
        );
    }

    disconnect() {
        this.panzoom && this.panzoom.destroy();
    }
}

window.application.register('mermaid-schema-vis', MermaidSchemaVisController);
