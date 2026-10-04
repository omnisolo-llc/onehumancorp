import React from 'react';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import GooseMcpPage from './page';

const unavailable = 'Agent runtime is not configured; no work was dispatched';
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });
it('shows unavailable runtime truthfully without presenting unread extensions as empty', async () => {
  const response = Response.json({ error: unavailable }, { status: 503 });
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(response));
  render(<GooseMcpPage />);
  expect(await screen.findByRole('alert')).toHaveTextContent(unavailable);
  expect(response.bodyUsed).toBe(true);
  expect(screen.queryByText('No extensions found.')).toBeNull();
  expect(screen.queryByRole('button', { name: 'Execute' })).toBeNull();
});
it('reports empty only after a verified successful extension list', async () => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ result: [] })));
  render(<GooseMcpPage />);
  expect(await screen.findByText('No extensions found.')).toBeVisible();
  expect(screen.queryByRole('alert')).toBeNull();
});
it.each([
  { result: [{ id: 'extension-a', name: 'Actual extension' }], status: 503 },
  { result: 'not a list', status: 200 },
  { result: [{ id: '', name: 'Unbound extension' }], status: 200 },
])('does not offer execution for an unverified list %j', async ({ result, status }) => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ result }, { status })));
  render(<GooseMcpPage />);
  expect(await screen.findByRole('alert')).toBeVisible();
  expect(screen.queryByText('No extensions found.')).toBeNull();
  expect(screen.queryByRole('button', { name: 'Execute' })).toBeNull();
});
it('retires the previous extension list when its refresh cannot be verified', async () => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValueOnce(Response.json({ result: [{ id: 'extension-a', name: 'Actual extension' }] }))
    .mockResolvedValueOnce(Response.json({ error: unavailable }, { status: 503 })));
  render(<GooseMcpPage />);
  expect(await screen.findByRole('button', { name: 'Execute' })).toBeVisible();
  fireEvent.click(screen.getByRole('button', { name: 'Refresh List' }));
  await waitFor(() => expect(screen.queryByRole('button', { name: 'Execute' })).toBeNull());
  expect(await screen.findByRole('alert')).toHaveTextContent(unavailable);
});
