import { cleanup, render, screen, fireEvent, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, test, vi } from 'vitest';
import TeamChatPage from './page';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../../onboarding/testLocks';
let response: () => Promise<Response>;
beforeEach(() => {
  localStorage.clear(); invalidateQueueOwner(); installOnboardingLocks();
  vi.stubGlobal('fetch', vi.fn(async (url) => String(url).endsWith('/session-identity')
    ? Response.json({ userId: 'owner-a', tenantId: 'tenant-a', expiresAt: Date.now() + 60000 })
    : String(url).startsWith('/api/v1/agents/approvals') ? Response.json({ pending_approvals: [], next_cursor: null }) : response()));
});
afterEach(() => { cleanup(); invalidateQueueOwner(); vi.unstubAllGlobals(); });
async function send() {
  render(<TeamChatPage />);
  await waitFor(() => expect(screen.getByTestId('team-chat-input')).toBeEnabled());
  fireEvent.change(screen.getByTestId('team-chat-input'), { target: { value: 'Quote the sink repair' } });
  fireEvent.click(screen.getByTestId('team-chat-send'));
}
test('shows pending acceptance without claiming a drafted business action', async () => {
  let finish!: (value: Response) => void; response = () => new Promise(resolve => { finish = resolve; });
  await send(); expect(await screen.findByText('Saving your request for department review…')).toBeVisible();
  finish(Response.json({ success: true, department_assigned: 'sales', approval: { id: 'stored-quote', tenant_id: 'tenant-a', department: 'Sales', description: 'Task routed via semantic gateway to Sales', status: 'PendingApproval', action_risk: 'DraftForReview', payload: { original_request: 'Quote the sink repair', action: 'semantic_routed_task' } } }));
  expect(await screen.findByText('Task routed via semantic gateway to Sales')).toBeVisible();
  expect(screen.queryByText("I've drafted an action for your approval.")).toBeNull();
});
test('shows a definitive rejection and keeps the unsent prompt available', async () => {
  response = async () => Response.json({ error: 'AI Budget exhausted' }, { status: 429 });
  await send(); expect(await screen.findByText('Request rejected (HTTP 429). AI Budget exhausted')).toBeVisible();
  expect(screen.getByTestId('team-chat-input')).toHaveValue('Quote the sink repair');
  expect(screen.queryByRole('button', { name: 'Record approval' })).toBeNull();
});
