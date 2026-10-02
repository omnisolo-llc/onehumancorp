import { fireEvent, render, screen } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import { UnifiedAgentFeed } from './UnifiedAgentFeed';
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
  expect(screen.getByText('Review this local note')).toBeVisible(); expect(input).toHaveValue(''); expect(send).toBeDisabled();
  expect(fetcher.mock.calls.filter((call: unknown[]) => (call[1] as RequestInit | undefined)?.method === 'POST')).toHaveLength(0);
});
