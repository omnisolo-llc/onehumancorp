import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
const source=await readFile(new URL('../src/server/api/staff_mesh.rs',import.meta.url),'utf8');
test('mounted staff, task and summary reads never turn SQL errors into empty records',()=>{
 for(const [name,next] of [['get_staff_handler','sync_timecard_handler'],['get_tasks_handler','update_task_handler'],['get_summaries_handler','get_shifts_handler']]){
  const start=source.indexOf(`pub async fn ${name}(`);const end=source.indexOf(`pub async fn ${next}(`,start);assert.ok(start>=0&&end>start);
  const body=source.slice(start,end);assert.doesNotMatch(body,/unwrap_or_default\(\)/);assert.match(body,/StatusCode::INTERNAL_SERVER_ERROR/);
 }
});
test('native PostgreSQL migrations include the actual staff mesh schema without changing its historical SQL',async()=>{
 const historical=await readFile(new URL('../src/server/db/migrations/016_staff_mesh.sql',import.meta.url),'utf8');
 const current=await readFile(new URL('../src/server/migrations/1027_staff_mesh_native.sql',import.meta.url),'utf8');
 assert.ok(current.startsWith(historical));for(const table of ['ohc_staff_member','ohc_timecard_event'])assert.ok(current.includes(`ALTER TABLE ${table} FORCE ROW LEVEL SECURITY;`));
});
test('unused staff demo routes cannot persist simulated events or fabricated AI summaries',()=>{
 const routing=source.slice(source.indexOf('pub fn router<'),source.indexOf('#[cfg(test)]',source.indexOf('pub fn router<')));
 for(const path of ['/simulate-event','/generate-summary'])assert.ok(!routing.includes(`"${path}"`));
 assert.doesNotMatch(source,/pub async fn (?:simulate_event_handler|generate_summary_handler)\(/);
 assert.ok(!source.includes('Simulated AI Summary'));
 for(const path of ['/tasks','/timecard','/summaries','/shifts','/escalations'])assert.ok(routing.includes(`"${path}"`));
});
