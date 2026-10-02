'use client';

import { useRef, useState, type FormEvent } from 'react';
import { useRouter } from 'next/navigation';
import { assertBuilderEditor, readBuilderDraft, writeBuilderDraft } from '../../builder/ownedDraft';
import { readPublication, type Publication } from '../definitions';
import { useDefinitionSession } from '../useDefinitionSession';
import { OperationRecovery } from '../OperationRecovery';

const DRAFT = 'agent-publication-draft';
const empty = (): Publication => ({ name: '', description: '', role: '', system_prompt: '', visibility: 'public' });
function readDraft(value: unknown): Publication {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('The saved publication draft remains held.');
  const data = value as Record<string, unknown>;
  if (data.visibility !== 'public' || Object.keys(data).some(key => !['name', 'description', 'role', 'system_prompt', 'visibility'].includes(key))) throw new Error('The saved publication draft remains held.');
  for (const [key, max] of [['name',120],['description',2000],['role',120],['system_prompt',16000]] as const) {
    if (typeof data[key] !== 'string' || [...data[key]].length > max || data[key].includes('\0')) throw new Error('The saved publication draft remains held.');
  }
  return { name: data.name as string, description: data.description as string, role: data.role as string, system_prompt: data.system_prompt as string, visibility: 'public' };
}
export default function PublishAgentPage() {
  const router = useRouter();
  const [draft, setDraft] = useState<Publication>(empty);
  const [review, setReview] = useState<Publication | null>(null);
  const [draftError, setDraftError] = useState('');
  const [storageHeld, setStorageHeld] = useState(false);
  const submitted = useRef(false);
  const session = useDefinitionSession({ editor: DRAFT,
    opened: scope => {
      try { const saved = readBuilderDraft<Publication>(DRAFT, scope); setDraft(saved ? readDraft(saved.data) : empty()); }
      catch (cause) { setStorageHeld(true); setDraftError('The original saved publication draft could not be read and remains held.'); throw cause; }
    },
    retired: () => { setDraft(empty()); setReview(null); setDraftError(''); setStorageHeld(false); submitted.current = false; },
  });
  const canEdit = session.phase === 'ready' && !storageHeld;
  function change(field: keyof Omit<Publication, 'visibility'>, value: string) {
    if (!canEdit) return;
    const next = { ...draft, [field]: value }; setDraft(next); setReview(null);
    try { readDraft(next); } catch { setDraftError('Keep name and role within 120 characters, description within 2000, and prompt within 16000, without null characters.'); return; }
    try {
      if (!session.scope.current) throw new Error('Verified publication access is unavailable.');
      writeBuilderDraft(DRAFT, next, session.scope.current); setDraftError('');
    } catch (error) { setStorageHeld(true); setDraftError(error instanceof Error ? error.message : 'Your latest edits could not be saved on this device.'); }
  }
  function prepare(event: FormEvent) {
    event.preventDefault(); if (!canEdit || draftError) return;
    try {
      if (!session.scope.current) throw new Error('Verified publication access is unavailable.');
      assertBuilderEditor(session.scope.current, DRAFT); setReview(readPublication(draft));
    } catch (error) { session.setMessage(error instanceof Error ? error.message : 'Review the publication fields.'); }
  }
  async function publish() {
    if (!canEdit || draftError || !review || submitted.current) return;
    submitted.current = true; const scope = session.scope.current;
    try {
      const receipt = await session.perform({ kind: 'publish', request_id: crypto.randomUUID(), publication: review });
      if (receipt?.status === 'published') { setReview(null); router.push('/agent-marketplace'); }
    } finally { if (session.scope.current === scope) submitted.current = false; }
  }
  async function replay() {
    const pending = session.saved;
    if (pending?.state !== 'pending' || session.phase === 'working') return;
    const receipt = await session.perform(pending.operation, true);
    if (receipt?.status === 'published') { setReview(null); router.push('/agent-marketplace'); }
  }
  return <main className="min-h-screen bg-[#f4f6f8] p-8 font-outfit">
    <div className="max-w-3xl mx-auto">
      <h1 className="text-4xl font-bold mb-2">Publish New Agent</h1>
      <p className="mb-6">Prepare a versioned agent definition, then review its public fields before publication.</p>
      <p role="status" className="mb-4">{session.message}</p>
      {draftError && <p role="alert">{draftError} Your current entries remain in this view. Reload only after preserving them.</p>}
      {(session.saved?.state === 'pending' || session.phase === 'held') && <OperationRecovery saved={session.saved} available={!!session.scope.current} busy={session.phase === 'working'} recover={() => void session.perform()} replay={() => void replay()} />}
      <form onSubmit={prepare} className="bg-white rounded-xl p-8 space-y-6">
        <fieldset disabled={!canEdit} className="space-y-6">
          {([['name','Agent Name'],['description','Description'],['role','Role'],['system_prompt','System Prompt']] as const).map(([key,label]) => <div key={key}>
            <label htmlFor={key} className="block font-semibold mb-2">{label}</label>
            {key === 'system_prompt' ? <textarea id={key} value={draft[key]} onChange={event => change(key,event.target.value)} rows={8} required className="w-full border rounded-lg p-3" /> : <input id={key} value={draft[key]} onChange={event => change(key,event.target.value)} required={key !== 'description'} className="w-full border rounded-lg p-3" />}
          </div>)}
          <button type="submit" disabled={!!draftError} className="rounded-lg bg-blue-600 text-white px-6 py-3">Review Publication</button>
        </fieldset>
      </form>
      {review && <section aria-label="Public agent definition review" className="mt-6 bg-white rounded-xl p-8 space-y-4">
        <h2 className="text-2xl font-semibold">Review public definition</h2>
        <p>These fields will be visible to other authenticated users of this OHC marketplace, including other businesses. This does not publish to an external registry or start any work.</p>
        <dl>{([['name','Name'],['description','Description'],['role','Role'],['system_prompt','System Prompt']] as const).map(([key,label]) => <div key={key}><dt className="font-semibold">{label}</dt><dd className="whitespace-pre-wrap break-words">{review[key] || '(empty)'}</dd></div>)}</dl>
        <p>The saved definition is immutable. Installing it later saves an inactive copy; Hire is a separate authorized action.</p>
        <button type="button" disabled={!canEdit} onClick={() => void publish()} className="rounded-lg bg-blue-600 text-white px-6 py-3">Publish Publicly</button>
        <button type="button" disabled={!canEdit} onClick={() => setReview(null)} className="ml-4">Back to Editing</button>
      </section>}
    </div>
  </main>;
}
