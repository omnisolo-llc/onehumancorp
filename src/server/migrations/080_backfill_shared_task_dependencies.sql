-- Backfill normalized dependency edges from legacy JSON arrays.
INSERT INTO shared_task_dependencies (task_id, depends_on_task_id, organization_id)
SELECT st.id::text, dep.depends_on_task_id, st.organization_id::text
FROM shared_tasks st
CROSS JOIN LATERAL jsonb_array_elements_text(COALESCE(st.dependencies, '[]'::jsonb)) AS dep(depends_on_task_id)
WHERE dep.depends_on_task_id IS NOT NULL
  AND dep.depends_on_task_id <> ''
ON CONFLICT DO NOTHING;

INSERT INTO shared_task_dependencies (task_id, depends_on_task_id, organization_id)
SELECT std.id::text, dep.depends_on_task_id, std.organization_id::text
FROM shared_tasks_decomposition std
CROSS JOIN LATERAL jsonb_array_elements_text(COALESCE(std.dependencies, '[]'::jsonb)) AS dep(depends_on_task_id)
WHERE dep.depends_on_task_id IS NOT NULL
  AND dep.depends_on_task_id <> ''
ON CONFLICT DO NOTHING;

UPDATE shared_task_dependencies dep
SET organization_id = st.organization_id::text
FROM shared_tasks st
WHERE dep.task_id = st.id::text
  AND dep.organization_id IS NULL;
