import { Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
import { PGlite } from 'https://cdn.jsdelivr.net/npm/@electric-sql/pglite@0.2.15/dist/index.js';

class DbController extends Controller {
    static targets = ['schema'];

    async connect() {
        const schemaCreationQueries = this.getSchema();

        if (schemaCreationQueries) {
            await this.resetDb(schemaCreationQueries);
        }
    }

    getSchema() {
        // check the value first (bc textarea has both value and textContent)
        return (
            this.hasSchemaTarget &&
            (this.schemaTarget.value || this.schemaTarget.textContent)
        );
    }

    async schemaUpdated({
        detail: {
            schema: { schema },
        },
    }) {
        await this.resetDb(schema);
    }

    async resetDbRequest() {
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
        this.dbInfo = await this.getDbInfo();

        this.dispatch('db-created', { detail: { dbInfo: this.dbInfo } });
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

    async getDbInfo(schema = 'public') {
        const db = this.db;

        const entities = await this.getEntities(db, schema);
        const relationships = await this.getRelationships(db, schema);
        const indexes = await this.getIndexes(db, schema);

        return { entities, relationships, indexes };
    }

    // #region DB Info
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
        const relationships = await db.query(
            this.getRelationshipsQuery(schema),
        );

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
    // #endregion
}

window.application.register('db', DbController);
