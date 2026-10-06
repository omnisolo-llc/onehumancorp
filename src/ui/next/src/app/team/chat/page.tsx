'use client';
import { useEffect, useState } from 'react';
import Link from 'next/link';
import { useTeamApprovals } from '../useTeamApprovals';

export default function TeamChatPage() {
  const team = useTeamApprovals();
  const [message, setMessage] = useState('');
  useEffect(() => { setMessage(''); }, [team.revision]);
  const send = async () => {
    const submitted = message;
    if (await team.send(submitted)) setMessage(current => current === submitted ? '' : current);
  };
  return (
    <div className="min-h-screen bg-gray-50 p-4 font-inter">
      <main className="mx-auto max-w-2xl space-y-4 rounded-2xl border border-gray-200 bg-white p-6">
        <header>
          <Link href="/team" className="text-blue-700 underline">Back to Team</Link>
          <h1 className="mt-3 text-2xl font-bold">Team Chat</h1>
          <p className="text-sm text-gray-600">Save a request for department review. Approval and external execution are tracked separately.</p>
        </header>
        {team.error && <p role="alert">{team.error}</p>}
        {team.loading && <p role="status">Loading recorded approvals…</p>}
        {team.chatNotice && <p role="status">{team.chatNotice}</p>}
        {[...team.notices].map(([id, notice]) => <p key={id} role="status" data-approval-id={id}>{notice.message}</p>)}
        <button type="button" onClick={() => void team.refresh()} disabled={team.busy} className="min-h-[44px] text-blue-700 underline">Refresh recorded decisions</button>
        {team.ready && !team.loading && !team.error && team.items.length === 0 && team.notices.size === 0 && <p>No pending approvals were returned.</p>}
        <section hidden={!team.ready} aria-label="Pending department approvals" className="space-y-4">
          {team.items.map(item => (
            <article key={item.id} data-testid="action-card" className="rounded-xl border border-gray-200 p-4">
              <p className="text-xs font-semibold uppercase text-gray-600">{item.department.replaceAll('_', ' ')} · {item.action_risk === 'HIGH' ? 'Owner review required' : 'Review'}</p>
              <h2 className="mt-2 font-semibold">{item.description}</h2>
              {item.payload && <pre className="my-3 whitespace-pre-wrap break-words text-xs">{JSON.stringify(item.payload, null, 2)}</pre>}
              <div className="mt-4 flex flex-wrap gap-3">
                <button type="button" disabled={team.blocked(item.id)} onClick={() => void team.decide(item.id, 'APPROVED')} className="min-h-[44px] rounded-lg bg-blue-700 px-4 text-white disabled:opacity-50">Record approval</button>
                <button type="button" disabled={team.blocked(item.id)} onClick={() => void team.decide(item.id, 'DISMISSED')} className="min-h-[44px] rounded-lg border px-4 disabled:opacity-50">Record dismissal</button>
                <Link href="/team" className="self-center text-blue-700 underline">Review or edit in Team</Link>
              </div>
            </article>
          ))}
        </section>
        <form onSubmit={event => { event.preventDefault(); void send(); }} className="flex gap-2 border-t pt-4">
          <input aria-label="Message your team" data-testid="team-chat-input" placeholder="Message your team..." value={team.ready ? message : ''} onChange={event => setMessage(event.target.value)} disabled={!team.ready || team.busy} maxLength={16000} className="min-w-0 flex-1 rounded-lg border px-3" />
          <button data-testid="team-chat-send" disabled={!team.ready || team.busy || !!team.error || !message.trim()} className="min-h-[44px] rounded-lg bg-blue-700 px-4 text-white disabled:opacity-50">Send</button>
        </form>
      </main>
    </div>
  );
}
