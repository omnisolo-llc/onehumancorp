import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import type { ReactNode } from 'react';
import SettingsPage from './page';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main> }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: ReactNode }) => children }));
let provision: () => Promise<Response>;
beforeEach(() => {
  provision = async () => Response.json({ success: false, error: 'Provider is not configured' });
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => {
    if (url === '/api/v1/settings/voice/provision') return provision();
    if (url === '/api/v1/settings/voice' && options?.method !== 'POST') return Response.json({ voice_receptionist_enabled: true, voice_receptionist_number: '', voice_receptionist_persona: 'Friendly', voice_receptionist_instructions: '' });
    return Response.json({});
  }));
  vi.spyOn(console, 'error').mockImplementation(() => {});
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
it('an HTTP200 rejection cannot install or save a provisioned number', async () => {
  render(<SettingsPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Get Number' }));
  expect(await screen.findByText(/number could not be provisioned/i)).toBeVisible();
  expect(screen.getByLabelText('Assigned Phone Number')).toHaveValue('Not assigned');
  expect(vi.mocked(fetch).mock.calls.filter(([url, options]) => url === '/api/v1/settings/voice' && options?.method === 'POST')).toHaveLength(0);
});
it('an acknowledged number is displayed without a second settings mutation', async () => {
  provision = async () => Response.json({ success: true, number: '+14155550123' });
  render(<SettingsPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Get Number' }));
  await waitFor(() => expect(screen.getByLabelText('Assigned Phone Number')).toHaveValue('+14155550123'));
  expect(vi.mocked(fetch).mock.calls.filter(([url, options]) => url === '/api/v1/settings/voice' && options?.method === 'POST')).toHaveLength(0);
});
it.each(['missing', 'malformed', 'contradictory'])('an acknowledged-looking %s number remains unconfirmed', async kind => {
  provision = async () => Response.json({ success: true, ...(kind === 'malformed' ? { number: 'not-a-phone-number' } : kind === 'contradictory' ? { number: '+14155550123', error: 'Provisioning was rejected' } : {}) });
  render(<SettingsPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Get Number' }));
  expect(await screen.findByText(/number could not be provisioned/i)).toBeVisible();
  expect(screen.getByLabelText('Assigned Phone Number')).toHaveValue('Not assigned');
  expect(vi.mocked(fetch).mock.calls.filter(([url, options]) => url === '/api/v1/settings/voice' && options?.method === 'POST')).toHaveLength(0);
});
it('an uncertain provisioning reply holds repeated clicks in the current view', async () => {
  provision = async () => { throw new Error('Reply lost'); };
  render(<SettingsPage />);
  const button = await screen.findByRole('button', { name: 'Get Number' });
  fireEvent.click(button);
  expect(await screen.findByText(/number could not be provisioned/i)).toBeVisible();
  expect(button).toBeDisabled();
  fireEvent.click(button);
  expect(vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/settings/voice/provision')).toHaveLength(1);
  expect(screen.getByLabelText('Assigned Phone Number')).toHaveValue('Not assigned');
});
