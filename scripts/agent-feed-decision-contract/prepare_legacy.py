"""Add exact legacy-triage production code and schemas to the existing DB gate."""
import json
import re


def extend_legacy_contract(root, here, paths, lines, read):
    module = root / 'src/server/api/legacy_triage.rs'
    assert module.is_file(), 'Legacy triage must expose its actual production decision handler'
    paths.append(module)
    registration = read('src/server/api/mod.rs')
    assert re.search(r'^pub mod legacy_triage;$', registration, re.M)
    mounted = read('src/server/lib.rs')
    for route in ['/api/v1/ui/triage/action', '/api/v1/triage/action']:
        assert re.search(
            re.escape(f'.route("{route}",')
            + r'[^\n]*post\(crate::api::legacy_triage::action\)[^\n]*Extension\(http_auth_store.clone\(\)\)',
            mounted,
        ), f'Legacy compatibility route must mount the actual canonical-auth handler: {route}'
    assert re.search(
        re.escape('.route("/api/v1/ui/triage/decisions/{id}",')
        + r'[^\n]*get\(crate::api::legacy_triage::read_decision\)[^\n]*Extension\(http_auth_store.clone\(\)\)',
        mounted,
    ), 'The read-only legacy decision reconciliation route must be mounted with canonical authority'
    lines.append(f'#[path={json.dumps(str(module))}]pub mod legacy_triage;')
    lines.append('pub async fn invalidate_legacy_triage_caches(_: &str) {}')
    task_queries = []
    for source in [mounted, read('src/server/api/work_triage.rs')]:
        task_queries.extend(re.findall(r'"(SELECT [^"\n]* FROM task_envelopes WHERE [^"\n]*)"', source))
    assert len(task_queries) == 8, 'Preserve every full/mobile task pending projection'
    lines.append('pub const LEGACY_TASK_READ_QUERIES: &[&str] = &['
                 + ','.join(json.dumps(query) for query in task_queries) + '];')
    inbox_queries = re.findall(
        r"^\s*(SELECT id, tenant_id, COALESCE\(source, 'omni_inbox'\)[^\n]* FROM omni_inbox_messages WHERE[^\n]*)$",
        mounted, re.M,
    )
    assert len(inbox_queries) == 2, 'Preserve both production inbox pending projections'
    for engine, query in zip(['PG', 'SQLITE'], inbox_queries):
        lines.append(f'pub const LEGACY_INBOX_PENDING_{engine}: &str = {json.dumps(query)};')

    # Copy complete table definitions and their RLS policies from real migrations.
    # No replacement database implementation or synthetic successful dispatcher.
    parity = read('src/server/migrations/1014_feature_parity_and_runtime_contract.sql')
    postgres = [read('src/server/migrations/109_triage_items.sql'),
                read('src/server/migrations/200_department_handoff.sql')]
    for filename, table in [('001_initial.sql', 'bookings'),
                            ('008_data_model_architecture.sql', 'services')]:
        source = read('src/server/migrations/' + filename)
        definition = re.search(rf'CREATE TABLE IF NOT EXISTS {table} \(.*?\n\);', source, re.S)
        assert definition is not None
        postgres.append(definition.group())
    for filename in ['078_quote_engine.sql',
                     '1001_create_omni_inbox_messages_and_quotes_fix.sql',
                     '1002_add_created_updated_at_to_quotes.sql',
                     '1015_bookings_service_id_and_compat.sql',
                     '1016_quote_line_items_parity.sql',
                     '163_shift_coordination.sql', '1027_staff_mesh_native.sql']:
        postgres.append(read('src/server/migrations/' + filename))
    for name in ['unified_threads', 'unified_triage_actions', 'inbound_signals', 'daily_work_items']:
        match = re.search(
            rf'CREATE TABLE IF NOT EXISTS {name} \(.*?WITH CHECK\s*\(.*?;',
            parity, re.S,
        )
        assert match is not None, f'Production PostgreSQL schema missing: {name}'
        postgres.append(match.group())
    migrations = [path for path in (root / 'src/server/migrations').glob('*.sql')
                  if re.search(r'CREATE TABLE(?: IF NOT EXISTS)? legacy_triage_decisions\s*\(', path.read_text())]
    assert len(migrations) == 1, 'Exactly one production legacy-decision migration is required'
    postgres.append(read(str(migrations[0].relative_to(root))))
    (here / 'legacy_schema.sql').write_text('\n'.join(postgres) + '\n')

    sqlite_source = read('src/server/db.rs')
    sqlite_tables = []
    for name in ['tenants', 'users', 'daily_work_items', 'triage_items',
                 'triage_proposed_actions', 'unified_threads', 'unified_triage_actions',
                 'omni_inbox_messages', 'inbox_messages', 'agent_feed_items',
                 'orders', 'legacy_triage_decisions', 'customers', 'quotes',
                 'quote_line_items', 'bookings', 'services', 'products',
                 'staff_profiles', 'shifts']:
        match = re.search(rf'CREATE TABLE IF NOT EXISTS {name} \(.*?\n\s*\);', sqlite_source, re.S)
        assert match is not None, f'Production SQLite schema missing: {name}'
        sqlite_tables.append(match.group())
    (here / 'legacy_sqlite_schema.sql').write_text('\n'.join(sqlite_tables) + '\n')
