type RecordValue = Record<string, unknown>;
const decoder = new TextDecoder('utf-8', { fatal: true });
const encoder = new TextEncoder();
const uuid = /^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/;
const record = (value: unknown): RecordValue | null => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as RecordValue : null;
const text = (value: unknown, fallback = ''): string => typeof value === 'string' ? value : fallback;
export function toBackendTask(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const input = record(JSON.parse(decoder.decode(body)));
  if (!input || !text(input.prompt).trim() || [...text(input.prompt)].length > 16000) throw new Error('invalid text task');
  const allowed = ['prompt','workspace','constraints','mode','model','provider','outputFormat','workDirectory','permissionProfile'];
  if (Object.keys(input).some(key => !allowed.includes(key)) || Object.values(input).some(value => typeof value !== 'string')) throw new Error('unsupported task fields');
  return encoder.encode(JSON.stringify(input));
}
export function toUiTask(value: unknown, legacy = false): RecordValue | null {
  const task = record(value);
  if (!task || !text(task.id) || !text(task.title)) return null;
  const config = record(task.model_config_json) ?? {};
  const execution = record(task.execution);
  const expected = {queued:'queued',dispatching:'running',completed:'completed',cancelled:'cancelled',outcome_unknown:'outcome_unknown'} as const;
  if (!legacy && (!execution || !uuid.test(text(execution.id)) || !uuid.test(text(execution.request_id))
    || !(text(execution.phase) in expected) || task.status !== expected[text(execution.phase) as keyof typeof expected]
    || (execution.phase === 'completed' ? !text(execution.output).trim() : execution.output != null))) return null;
  const createdAt = Number(task.created_at_unix), updatedAt = Number(task.updated_at_unix);
  const output = legacy ? null : text(execution?.output) || null;
  const prompt = text(task.prompt);
  return {
    id: text(task.id), title: text(task.title), workspace: text(task.workspace_id, 'Personal OS'),
    status: legacy ? 'blocked' : task.status,
    currentStep: legacy ? 'Legacy record: no admitted execution receipt' : text(task.current_step),
    mode: text(task.mode, 'Ask'), model: text(config.model, 'Auto'), provider: text(config.provider, 'Auto'),
    permissionProfile: legacy ? text(task.permission_profile) : 'Text only', riskSummary: [], artifacts: [], changes: [],
    messages: [...(prompt ? [{id:`${text(task.id)}-prompt`,role:'user',content:prompt}] : []), ...(output ? [{id:`${text(execution?.id)}-output`,role:'assistant',content:output}] : [])],
    execution: legacy ? null : execution, output, matchedRequestId: text(task.matched_request_id) || undefined, archived: task.archived === true, legacy,
    createdAt: Number.isFinite(createdAt) && createdAt > 0 ? new Date(createdAt * 1000).toISOString() : undefined,
    updatedAt: Number.isFinite(updatedAt) && updatedAt > 0 ? new Date(updatedAt * 1000).toISOString() : undefined,
  };
}
