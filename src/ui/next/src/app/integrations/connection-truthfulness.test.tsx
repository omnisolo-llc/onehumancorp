import { act, fireEvent, render, screen, within, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Integrations from './page';

const push = vi.fn();
vi.mock('next/navigation', () => ({ useRouter: () => ({ push }), usePathname: () => '/integrations' }));
vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: React.PropsWithChildren) => <main>{children}</main> }));
vi.mock('./ProviderConnections', () => ({ default: () => null }));
beforeEach(() => {
  push.mockReset();
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ success: true, integrations: [] })));
});
afterEach(() => { vi.unstubAllGlobals(); vi.unstubAllEnvs(); vi.restoreAllMocks(); });
function card(name: string): HTMLElement {
  const heading = screen.getByRole('heading', { name });
  const result = heading.closest('div.rounded-2xl');
  if (!result) throw new Error('Integration card not found');
  return result as HTMLElement;
}
it.each(['Ayrshare', 'Cal.com', 'Mercado Pago', 'Whereby', 'Resend', 'Front'])('does not infer %s connection from prompt text', async name => {
  const prompt = vi.spyOn(window, 'prompt').mockReturnValue('synthetic-not-a-credential');
  render(<Integrations />); await waitFor(() => expect(fetch).toHaveBeenCalledWith('/api/v1/integrations'));
  fireEvent.click(within(card(name)).getByRole('button', { name: 'Connect' }));
  expect(within(card(name)).getByRole('button', { name: 'Connect' })).toBeVisible();
  expect(screen.getByText(new RegExp(name.replace('.', '\\.') + ' connection is unavailable'))).toBeVisible();
  expect(prompt).not.toHaveBeenCalled(); expect(push).not.toHaveBeenCalled();
  expect(vi.mocked(fetch).mock.calls.some(([, options]) => options?.method === 'POST')).toBe(false);
});
it('the shipped Twilio UI cannot acknowledge a blank connection without a backend request', async () => {
  vi.stubEnv('NODE_ENV', 'production');
  render(<Integrations />); await waitFor(() => expect(fetch).toHaveBeenCalledWith('/api/v1/integrations'));
  fireEvent.click(within(card('Twilio Conversations')).getByRole('button', { name: 'Connect' }));
  const save = screen.getByRole('button', { name: 'Save & Connect' });
  expect(save).toBeDisabled(); fireEvent.click(save);
  expect(screen.queryByText('Twilio Conversations connected.')).toBeNull(); expect(push).not.toHaveBeenCalled();
  expect(vi.mocked(fetch).mock.calls.some(([, options]) => options?.method === 'POST')).toBe(false);
});
it.each([
  { status: 200, body: { success: true, status: 'connected', usable: true, error: 'verification failed' } },
  { status: 200, body: { success: true, status: 'pending', usable: false, integration: { status: 'connected', usable: true } } },
  { status: 202, body: { success: true, status: 'connected', usable: true } },
  { status: 200, body: { success: true, integration: { success: false, status: 'connected', usable: true } } },
  { status: 200, body: { success: true, status: 'connected', usable: true, integration: { success: false, status: 'connected', usable: true } } },
])('an unconfirmed Twilio response cannot install a connected state: %j', async fixture => {
  vi.mocked(fetch).mockImplementation(async url => String(url).endsWith('/twilio/connect')
    ? Response.json(fixture.body, { status: fixture.status })
    : Response.json({ success: true, integrations: [] }));
  render(<Integrations />); await waitFor(() => expect(fetch).toHaveBeenCalledWith('/api/v1/integrations'));
  fireEvent.click(within(card('Twilio Conversations')).getByRole('button', { name: 'Connect' }));
  fireEvent.change(screen.getByLabelText('Twilio Account SID'), { target: { value: 'synthetic-account' } });
  fireEvent.change(screen.getByLabelText('Twilio Auth Token'), { target: { value: 'synthetic-token' } });
  fireEvent.click(screen.getByRole('button', { name: 'Save & Connect' }));
  expect(await screen.findByText('Twilio Conversations connection could not be confirmed.')).toBeVisible();
  expect(screen.queryByText('Twilio Conversations connected.')).toBeNull(); expect(push).not.toHaveBeenCalled();
});
it('retains the existing nested verified receipt compatibility', async () => {
  vi.mocked(fetch).mockImplementation(async url => String(url).endsWith('/twilio/connect')
    ? Response.json({ success: true, integration: { status: 'connected', usable: true } })
    : Response.json({ success: true, integrations: [] }));
  render(<Integrations />); await waitFor(() => expect(fetch).toHaveBeenCalledWith('/api/v1/integrations'));
  fireEvent.click(within(card('Twilio Conversations')).getByRole('button', { name: 'Connect' }));
  fireEvent.change(screen.getByLabelText('Twilio Account SID'), { target: { value: 'synthetic-account' } });
  fireEvent.change(screen.getByLabelText('Twilio Auth Token'), { target: { value: 'synthetic-token' } });
  fireEvent.click(screen.getByRole('button', { name: 'Save & Connect' }));
  expect(await screen.findByText('Twilio Conversations connected.')).toBeVisible(); expect(push).toHaveBeenCalledWith('/inbox');
});
it('an integration-list error cannot advertise a usable connection', async () => {
  const response = Response.json({ success: true, error: 'verification failed', integrations: [{ id: 'ayrshare', status: 'connected', usable: true }] });
  vi.mocked(fetch).mockResolvedValue(response);
  await act(async () => { render(<Integrations />); });
  expect(response.bodyUsed).toBe(true);
  expect(within(card('Ayrshare')).getByRole('button', { name: 'Connect' })).toBeVisible();
  expect(within(card('Ayrshare')).getByText('disconnected', { exact: true })).toBeVisible();
});
