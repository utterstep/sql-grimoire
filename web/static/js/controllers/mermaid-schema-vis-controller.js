import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
import Panzoom from 'https://cdn.jsdelivr.net/npm/@panzoom/panzoom@4.5.1/+esm';

mermaid.initialize({
    startOnLoad: false,
    theme: 'dark',
});

export default class MermaidSchemaVisController extends Controller {
    static targets = ['schemaVis'];
    static outlets = ['db'];

    getEntitiesQuery(schema = 'public') {
        return `
    select
      columns.table_name as name,
      json_agg(
        json_build_object(
          'comment',
          case
            when columns.is_nullable = 'YES' then 'null'
            when columns.is_nullable = 'NO' then 'not null'
          end,
          'key',
          case
            when table_constraints.constraint_type = 'PRIMARY KEY' then 'PK'
            when table_constraints.constraint_type = 'FOREIGN KEY' then 'FK'
          end,
          'name',
          columns.column_name,
          'type',
          case
            when columns.data_type = 'ARRAY' then concat(regexp_replace(columns.udt_name, '^_', ''), '[]')
            when columns.data_type = 'USER-DEFINED' then columns.udt_name
            else replace(columns.data_type, ' ', '_')
          end
        )
        order by
          case
            when table_constraints.constraint_type = 'PRIMARY KEY' then 1
            when table_constraints.constraint_type = 'FOREIGN KEY' then 2
            else 3
          end,
          columns.is_nullable,
          case
            when columns.data_type = 'ARRAY' then concat(regexp_replace(columns.udt_name, '^_', ''), '[]')
            when columns.data_type = 'USER-DEFINED' then columns.udt_name
            else columns.data_type
          end,
          columns.column_name
      ) as attributes
      from
        information_schema.columns
        left join information_schema.key_column_usage
          on key_column_usage.table_schema = columns.table_schema
          and key_column_usage.table_name = columns.table_name
          and key_column_usage.column_name = columns.column_name
        left join information_schema.table_constraints
          on table_constraints.table_schema = key_column_usage.table_schema
          and table_constraints.table_name = key_column_usage.table_name
          and table_constraints.constraint_name = key_column_usage.constraint_name
      where
        columns.table_schema = '${schema}'
      group by
        name
      order by
        name;
    `;
    }

    async getEntities(db, schema = 'public') {
        const entities = await db.query(this.getEntitiesQuery(schema));

        return entities.rows;
    }

    getRelationshipsQuery(schema = 'public') {
        return `
    select
      json_build_object(
        'entity',
        child_key_column_usage.table_name,
        'attributes',
        json_agg(distinct child_key_column_usage.column_name)
      ) as child,
      json_build_object(
        'entity',
        parent_key_column_usage.table_name,
        'attributes',
        json_agg(distinct parent_key_column_usage.column_name)
      ) as parent
    from
      information_schema.referential_constraints
      join information_schema.key_column_usage as child_key_column_usage
        on child_key_column_usage.constraint_schema = referential_constraints.constraint_schema
        and child_key_column_usage.constraint_name = referential_constraints.constraint_name
      join information_schema.key_column_usage as parent_key_column_usage
        on parent_key_column_usage.constraint_schema = referential_constraints.unique_constraint_schema
        and parent_key_column_usage.constraint_name = referential_constraints.unique_constraint_name
    where
      referential_constraints.constraint_schema = '${schema}'
    group by
      child_key_column_usage.constraint_name,
      child_key_column_usage.table_name,
      parent_key_column_usage.table_name
    order by
      parent_key_column_usage.table_name,
      child_key_column_usage.table_name;
    `;
    }

    async getRelationships(db, schema = 'public') {
        const relationships = await db.query(this.getRelationshipsQuery(schema));

        return relationships.rows;
    }

    getIndexesQuery(schema = 'public') {
        return `
    select
      tablename as name,
      json_agg(indexname order by indexname) as indexes
    from
      pg_indexes
    where
      schemaname = '${schema}'
    group by
      tablename
    order by
      tablename;
    `;
    }

    async getIndexes(db, schema = 'public') {
        const indexes = await db.query(this.getIndexesQuery(schema));

        return indexes.rows;
    }

    async getDbData(db, schema = 'public') {
        const entities = await this.getEntities(db, schema);
        const relationships = await this.getRelationships(db, schema);
        const indexes = await this.getIndexes(db, schema);

        return { entities, relationships, indexes };
    }

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
    };

    generateDiagram({
        entities = [],
        relationships = []
    }) {
        const diagram = ['erDiagram'];

        if (entities.length > 0) {
            diagram.push(this.generateEntities(entities));
        }

        if (relationships.length > 0) {
            diagram.push(this.generateRelationships(relationships));
        }

        return diagram.join('\n\n');
    }

    async drawSchema(e) {
        const { entities, relationships, indexes: _indexes } = await this.getDbData(this.dbOutlet.db);
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
            maxScale: 2.5,
            step: 0.025,
        });
        this.schemaVisTarget.parentElement.addEventListener('wheel', this.panzoom.zoomWithWheel)
    }

    disconnect() {
        this.panzoom && this.panzoom.destroy();
    }
}

window.application.register('mermaid-schema-vis', MermaidSchemaVisController);
