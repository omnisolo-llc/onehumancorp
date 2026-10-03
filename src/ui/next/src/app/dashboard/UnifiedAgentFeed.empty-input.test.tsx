import { fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { UnifiedAgentFeed } from './UnifiedAgentFeed';
import { invalidateQueueOwner, readQueueOwner } from '@/lib/sync/queueIdentity';
beforeEach(async () => {
  invalidateQueueOwner();
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ userId: 'user-1', tenantId: 'tenant-1', expiresAt: Date.now() + 60_000 })));
  await readQueueOwner();
});
vi.mock('../utils/offlineQueue', () => ({ getActions: vi.fn().mockResolvedValue([]), enqueueAction: vi.fn(), removeAction: vi.fn() }));
afterEach(() => vi.unstubAllGlobals());
it.each(['', '   '])('does not offer Send for an empty dashboard message: %j', value => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ items: [] })));
  render(<UnifiedAgentFeed initialData={{ items: [], activity: [] }} />);
  fireEvent.change(screen.getByPlaceholderText('Message...'), { target: { value } });
  expect(screen.getByRole('button', { name: 'Send' })).toBeDisabled();
  expect(screen.queryByText('Understood.')).not.toBeInTheDocument();
});
it('retains entered text in the existing local view without claiming a backend receipt', () => {
  const fetcher = vi.fn().mockResolvedValue(Response.json({ items: [] })); vi.stubGlobal('fetch', fetcher);
  render(<UnifiedAgentFeed initialData={{ items: [], activity: [] }} />); const input = screen.getByPlaceholderText('Message...');
  fireEvent.change(input, { target: { value: '  Review this local note  ' } });
  const send = screen.getByRole('button', { name: 'Send' }); expect(send).toBeEnabled(); fireEvent.click(send);
  expect(input).toHaveValue('  Review this local note  ');
  expect(screen.getByRole('alert')).toHaveTextContent('Your draft has not been sent or saved');
  expect(screen.queryByText('Understood.')).not.toBeInTheDocument();
  expect(fetcher.mock.calls.filter((call: unknown[]) => (call[1] as RequestInit | undefined)?.method === 'POST')).toHaveLength(0);
});
