import type { SavedOperation } from './definitionOperations';

export function OperationRecovery({ saved, busy, available, recover, replay }: {
  saved: SavedOperation | null; busy: boolean; available: boolean; recover: () => void; replay: () => void;
}) {
  const operation = saved?.state === 'pending' ? saved.operation : null;
  const fields = operation?.kind === 'publish' ? operation.publication : operation?.definition;
  return <section aria-label="Saved agent operation recovery" className="my-6 rounded-lg border border-amber-300 p-4 space-y-3">
    {operation && fields && <>
      <h2>Unconfirmed {operation.kind === 'publish' ? 'public publication' : 'inactive installation'}</h2>
      <p>Request {operation.request_id}</p>
      {operation.kind === 'publish' ? <p>These exact fields were submitted for other authenticated users of this OHC marketplace, including other businesses.</p> : <p>This exact definition was submitted for an inactive installation. No work starts.</p>}
      <dl>{([['name','Name'],['description','Description'],['role','Role'],['system_prompt','System Prompt']] as const).map(([key,label]) => <div key={key}><dt className="font-semibold">{label}</dt><dd className="whitespace-pre-wrap break-words">{fields[key] || '(empty)'}</dd></div>)}</dl>
      {operation.kind === 'install' && <p>Version {operation.definition.version} · Digest {operation.definition.digest}</p>}
    </>}
    <button type="button" onClick={recover} disabled={!available || busy}>Check Saved Status</button>
    {operation && <button type="button" onClick={replay} disabled={!available || busy} className="ml-4">Retry Same Reviewed Request</button>}
    <p>A status check only reads saved receipts. An explicit retry keeps the original request identity and fields.</p>
  </section>;
}
