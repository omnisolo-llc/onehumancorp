import React from 'react';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import DynamicWorkflowsPage from './page';

const plan = {
  id: 'dwf-86d31ea5-34b6-461d-ab4a-5adb6d2aabcf', tenant_id: 'tenant-owned',
  prompt: 'Test dynamic workflow prompt', status: 'awaiting_confirmation',
  requires_confirmation: true, tasks: [{ id: 'task-1', title: 'Plan workflow shards' }],
};
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

function submit() {
  fireEvent.change(screen.getByRole('textbox'), { target: { value: plan.prompt } });
  fireEvent.click(screen.getByRole('button', { name: 'Generate Workflow' }));
}

it('renders the actual create envelope and refreshes the same owned plan', async () => {
  const fetcher = vi.fn().mockResolvedValueOnce(Response.json({ plan, enqueued_jobs: 0 }))
    .mockResolvedValueOnce(Response.json(plan));
  vi.stubGlobal('fetch', fetcher);
  render(<DynamicWorkflowsPage />);
  submit();
  await screen.findByRole('heading', { name: 'Workflow Status: awaiting_confirmation' });
  expect(screen.getByText('Plan saved. No work has been queued.')).toBeVisible();
  expect(JSON.parse(fetcher.mock.calls[0][1].body)).toEqual({ prompt: plan.prompt });
  fireEvent.click(screen.getByRole('button', { name: 'Refresh' }));
  await waitFor(() => expect(fetcher).toHaveBeenLastCalledWith(`/api/v1/dynamic-workflows/${plan.id}`));
  await waitFor(() => expect(screen.getByRole('button', { name: 'Approve & Queue Workflow' })).toBeEnabled());
});

it('keeps processing disabled while the request is pending and clears it on a real failure shape', async () => {
  let finish!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>(resolve => { finish = resolve; })));
  render(<DynamicWorkflowsPage />);
  submit();
  expect(screen.getByRole('button', { name: 'Processing...' })).toBeDisabled();
  finish(new Response('Failed to deserialize the JSON body into the target type', { status: 422 }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Failed to start workflow (HTTP 422)');
  expect(screen.getByRole('button', { name: 'Generate Workflow' })).toBeEnabled();
  expect(screen.queryByText(/Workflow Status:/)).toBeNull();
});

it.each([
  ['JSON error', () => Response.json({ error: 'prompt is required' }, { status: 400 }), 'prompt is required'],
  ['invalid success', () => Response.json({ id: plan.id, status: 'completed' }), 'Backend returned an invalid workflow response'],
  ['contradictory pending receipt', () => Response.json({ plan, enqueued_jobs: 4 }), 'Backend returned an invalid workflow response'],
  ['unparseable success', () => new Response('not JSON', { status: 200 }), 'Backend returned an invalid workflow response'],
] as const)('does not invent a plan for %s', async (_label, response, message) => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(response()));
  render(<DynamicWorkflowsPage />);
  submit();
  expect(await screen.findByRole('alert')).toHaveTextContent(message);
  expect(screen.queryByText(/Workflow Status:/)).toBeNull();
});

it('reports queue acknowledgement without claiming execution', async () => {
  const queued = { ...plan, status: 'queued', requires_confirmation: false };
  vi.stubGlobal('fetch', vi.fn().mockResolvedValueOnce(Response.json({ plan, enqueued_jobs: 0 }))
    .mockResolvedValueOnce(Response.json({ plan: queued, enqueued_jobs: 3 })));
  render(<DynamicWorkflowsPage />);
  submit();
  fireEvent.click(await screen.findByRole('button', { name: 'Approve & Queue Workflow' }));
  await screen.findByRole('heading', { name: 'Workflow Status: queued' });
  expect(screen.getByText('Workflow queued. Execution and completion are not verified.')).toBeVisible();
  expect(screen.queryByRole('button', { name: 'Approve & Queue Workflow' })).toBeNull();
});

it('rejects a refresh for a different plan instead of displaying its data', async () => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValueOnce(Response.json({ plan, enqueued_jobs: 0 }))
    .mockResolvedValueOnce(Response.json({ ...plan, tenant_id: 'other-tenant', prompt: 'foreign secret' })));
  render(<DynamicWorkflowsPage />);
  submit();
  fireEvent.click(await screen.findByRole('button', { name: 'Refresh' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Backend returned an invalid workflow response');
  expect(screen.queryByText(/foreign secret/)).toBeNull();
});

it('keeps confirmation unconfirmed through repeated awaiting-confirmation readbacks', async () => {
  const fetcher = vi.fn().mockResolvedValueOnce(Response.json({ plan, enqueued_jobs: 0 }))
    .mockRejectedValueOnce(new Error('connection lost'))
    .mockResolvedValueOnce(Response.json(plan))
    .mockResolvedValueOnce(Response.json(plan));
  vi.stubGlobal('fetch', fetcher);
  render(<DynamicWorkflowsPage />);
  submit();
  fireEvent.click(await screen.findByRole('button', { name: 'Approve & Queue Workflow' }));
  await screen.findByRole('alert');
  expect(screen.getByRole('button', { name: 'Approve & Queue Workflow' })).toBeDisabled();
  expect(screen.queryByText('Plan saved. No work has been queued.')).toBeNull();
  for (let readback = 0; readback < 2; readback += 1) {
    fireEvent.click(screen.getByRole('button', { name: 'Refresh' }));
    await waitFor(() => expect(screen.getByRole('button', { name: 'Refresh' })).toBeEnabled());
    expect(screen.getByRole('heading', { name: 'Workflow Status: awaiting_confirmation' })).toBeVisible();
    const confirm = screen.getByRole('button', { name: 'Approve & Queue Workflow' });
    expect(confirm).toBeDisabled();
    expect(screen.queryByText('Plan saved. No work has been queued.')).toBeNull();
    expect(screen.getByText('Confirmation outcome is unconfirmed. Approval remains disabled. Refresh to check its status.')).toBeVisible();
    fireEvent.click(confirm);
  }
  expect(fetcher.mock.calls.filter(([url]) => url === `/api/v1/dynamic-workflows/${plan.id}/confirm`))
    .toEqual([[`/api/v1/dynamic-workflows/${plan.id}/confirm`, { method: 'POST' }]]);
  expect(fetcher.mock.calls.filter(([url]) => url === `/api/v1/dynamic-workflows/${plan.id}`)).toHaveLength(2);
});

it.each(['queued', 'running', 'completed', 'failed'])('clears an unconfirmed outcome after a verified %s readback', async status => {
  const verified = { ...plan, status, requires_confirmation: false };
  const fetcher = vi.fn().mockResolvedValueOnce(Response.json({ plan, enqueued_jobs: 0 }))
    .mockRejectedValueOnce(new Error('connection lost'))
    .mockResolvedValueOnce(Response.json(verified));
  vi.stubGlobal('fetch', fetcher);
  render(<DynamicWorkflowsPage />);
  submit();
  fireEvent.click(await screen.findByRole('button', { name: 'Approve & Queue Workflow' }));
  await screen.findByRole('alert');
  expect(screen.getByText(/Confirmation outcome is unconfirmed/)).toBeVisible();
  fireEvent.click(screen.getByRole('button', { name: 'Refresh' }));
  await screen.findByRole('heading', { name: `Workflow Status: ${status}` });
  expect(screen.queryByText(/Confirmation outcome is unconfirmed/)).toBeNull();
  expect(screen.queryByRole('button', { name: 'Approve & Queue Workflow' })).toBeNull();
  expect(screen.queryByText('Plan saved. No work has been queued.')).toBeNull();
  expect(fetcher.mock.calls.filter(([url]) => url === `/api/v1/dynamic-workflows/${plan.id}/confirm`))
    .toEqual([[`/api/v1/dynamic-workflows/${plan.id}/confirm`, { method: 'POST' }]]);
});
