import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import type { ReactNode } from 'react';
import Integrations from './page';

const push = vi.fn();
vi.mock('next/navigation', () => ({ useRouter: () => ({ push }), usePathname: () => '/integrations' }));
vi.mock('../../components/TooltipRegistry', () => ({ TooltipProvider: ({ children }: { children?: ReactNode }) => children, WithTooltip: ({ children }: { children?: ReactNode }) => children }));
beforeEach(() => {
  push.mockClear();
  vi.stubGlobal('FB', undefined);
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url === '/api/v1/integrations'
    ? Response.json({ success: true, integrations: [] })
    : Response.json({ success: false, status: 'pending_verification', usable: false, message: 'Secure provider verification is unavailable.' }, { status: 501 })));
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
async function openCloudConnection() {
  render(<Integrations />);
  await waitFor(() => expect(fetch).toHaveBeenCalledWith('/api/v1/integrations'));
  const heading = screen.getByRole('heading', { name: 'WhatsApp Cloud API' });
  const card = heading.closest('div.rounded-2xl');
  expect(card).not.toBeNull();
  fireEvent.click(within(card as HTMLElement).getByRole('button', { name: 'Connect' }));
  fireEvent.click(screen.getByRole('button', { name: 'Continue with Meta' }));
}
it('an absent Meta SDK never manufactures a credential or sends a connect request', async () => {
  await openCloudConnection();
  await waitFor(() => expect(screen.getByText(/Meta sign-in is not configured/i)).toBeVisible());
  expect(vi.mocked(fetch).mock.calls.filter(([, options]) => options?.method === 'POST')).toHaveLength(0);
  expect(push).not.toHaveBeenCalled();
  expect(screen.queryByText('WhatsApp Cloud API connected.')).toBeNull();
});
it('an unverified backend response remains explicitly unavailable after the SDK callback', async () => {
  vi.stubGlobal('FB', { login: (callback: (value: unknown) => void) => callback({ authResponse: { accessToken: 'synthetic-unverified-input' } }) });
  await openCloudConnection();
  expect(await screen.findByText(/provider verification is unavailable/i)).toBeVisible();
  expect(screen.queryByText('WhatsApp Cloud API connected.')).toBeNull();
  expect(push).not.toHaveBeenCalled();
});
