import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import type { ReactNode } from 'react';
import SettingsPage from './page';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main> }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: ReactNode }) => children }));
let provision: () => Promise<Response>;
let readVoice: () => Response | Promise<Response>;
let saveVoice: () => Response | Promise<Response>;
beforeEach(() => {
  provision = async () => Response.json({ success: false, error: 'Provider is not configured' });
  readVoice = () => Response.json({ voice_receptionist_enabled: true, voice_receptionist_number: '', voice_receptionist_persona: 'Friendly', voice_receptionist_instructions: '', provisioning_available: true, provisioning_state: 'not_started', provisioning_block_reason: null });
  saveVoice = () => Response.json({ success: true });
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => {
    if (url === '/api/v1/settings/voice/provision') return provision();
    if (url === '/api/v1/settings/voice') return options?.method === 'POST' ? saveVoice() : readVoice();
    return Response.json({});
  }));
  vi.spyOn(console, 'error').mockImplementation(() => {});
});
it('hosted global voice settings remain explicitly unavailable', async () => {
  readVoice = () => Response.json({ success: false, error: 'hosted_global_provisioning_unavailable', provisioning_available: false }, { status: 403 });
  render(<SettingsPage />);
  expect(await screen.findByText(/voice settings are unavailable in this deployment/i)).toBeVisible();
  expect(screen.getByRole('checkbox', { name: 'Enable AI Voice Receptionist' })).toBeDisabled();
  expect(screen.queryByRole('button', { name: 'Get Number' })).toBeNull();
});
it.each(['provider_not_configured', 'provisioning_requires_reconciliation'])('server %s state prevents provisioning after page reload', async reason => {
  readVoice = () => Response.json({ voice_receptionist_enabled: true, voice_receptionist_number: '', voice_receptionist_persona: 'Friendly', voice_receptionist_instructions: '', provisioning_available: false, provisioning_state: 'pending', provisioning_block_reason: reason });
  const first = render(<SettingsPage />);
  expect(await screen.findByRole('button', { name: 'Get Number' })).toBeDisabled();
  first.unmount();
  render(<SettingsPage />);
  const button = await screen.findByRole('button', { name: 'Get Number' });
  expect(button).toBeDisabled();
  fireEvent.click(button);
  expect(vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/settings/voice/provision')).toHaveLength(0);
});
it('a rejected preference update does not become an enabled voice setting', async () => {
  readVoice = () => Response.json({ voice_receptionist_enabled: false, voice_receptionist_number: '', voice_receptionist_persona: 'Friendly', provisioning_available: false, provisioning_state: 'not_started', provisioning_block_reason: 'provider_not_configured' });
  saveVoice = () => Response.json({ success: false });
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable AI Voice Receptionist' });
  await waitFor(() => expect(toggle).not.toBeDisabled());
  fireEvent.click(toggle);
  expect(await screen.findByText(/voice preference change could not be confirmed/i)).toBeVisible();
  expect(toggle).not.toBeChecked();
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
it('an HTTP200 rejection cannot install or save a provisioned number', async () => {
  render(<SettingsPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Get Number' }));
  expect(await screen.findByText(/number could not be provisioned/i)).toBeVisible();
  expect(screen.getByLabelText('Phone number')).toHaveValue('Not assigned');
  expect(vi.mocked(fetch).mock.calls.filter(([url, options]) => url === '/api/v1/settings/voice' && options?.method === 'POST')).toHaveLength(0);
});
it('an acknowledged number is displayed without a second settings mutation', async () => {
  provision = async () => Response.json({ success: true, number: '+14155550123' });
  render(<SettingsPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Get Number' }));
  await waitFor(() => expect(screen.getByLabelText('Phone number')).toHaveValue('+14155550123'));
  expect(vi.mocked(fetch).mock.calls.filter(([url, options]) => url === '/api/v1/settings/voice' && options?.method === 'POST')).toHaveLength(0);
});
it.each(['missing', 'malformed', 'contradictory'])('an acknowledged-looking %s number remains unconfirmed', async kind => {
  provision = async () => Response.json({ success: true, ...(kind === 'malformed' ? { number: 'not-a-phone-number' } : kind === 'contradictory' ? { number: '+14155550123', error: 'Provisioning was rejected' } : {}) });
  render(<SettingsPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Get Number' }));
  expect(await screen.findByText(/number could not be provisioned/i)).toBeVisible();
  expect(screen.getByLabelText('Phone number')).toHaveValue('Not assigned');
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
  expect(screen.getByLabelText('Phone number')).toHaveValue('Not assigned');
});

it.each(['auth', 'epoch', 'clear'])('clears private voice settings immediately on %s invalidation', async kind => {
  readVoice = () => Response.json({ voice_receptionist_enabled: true, voice_receptionist_number: '+14155550123', voice_receptionist_persona: 'Private persona', provisioning_available: true });
  render(<SettingsPage />); expect(await screen.findByLabelText('Phone number')).toHaveValue('+14155550123');
  act(() => window.dispatchEvent(kind === 'auth' ? new Event('omnisolo_auth_changed') : new StorageEvent('storage', { key: kind === 'epoch' ? 'omnisolo_queue_identity_epoch_v2' : null })));
  expect(screen.queryByLabelText('Phone number')).toBeNull();
  expect(screen.getByRole('checkbox', { name: 'Enable AI Voice Receptionist' })).toBeDisabled();
});
it.each(['read', 'provision', 'preference'])('does not apply a late voice %s receipt after identity changes', async kind => {
  let reply!: (response: Response) => void;
  const delayed = () => new Promise<Response>(resolve => { reply = resolve; });
  if (kind === 'read') readVoice = delayed;
  if (kind === 'provision') provision = delayed;
  if (kind === 'preference') saveVoice = delayed;
  render(<SettingsPage />);
  if (kind === 'provision') fireEvent.click(await screen.findByRole('button', { name: 'Get Number' }));
  if (kind === 'preference') fireEvent.change(await screen.findByRole('combobox', { name: 'Voice Persona' }), { target: { value: 'Professional' } });
  await waitFor(() => expect(reply).toBeDefined());
  act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
  await act(async () => reply(Response.json(kind === 'read' ? { voice_receptionist_enabled: true, voice_receptionist_number: '+14155550123', provisioning_available: true } : { success: true, number: '+14155550123' })));
  expect(await screen.findByRole('checkbox', { name: 'Enable AI Voice Receptionist' })).toBeDisabled();
  expect(screen.queryByLabelText('Phone number')).toBeNull();
  expect(screen.queryByText('Voice preference saved.')).toBeNull();
  expect(screen.queryByText('Phone number provisioning was acknowledged.')).toBeNull();
});

it('sends only the changed voice preference field', async () => {
  render(<SettingsPage />);
  fireEvent.change(await screen.findByRole('combobox', { name: 'Voice Persona' }), { target: { value: 'Professional' } });
  await screen.findByText('Voice preference saved.');
  const call = vi.mocked(fetch).mock.calls.find(([url, options]) => url === '/api/v1/settings/voice' && options?.method === 'POST');
  expect(JSON.parse(String(call?.[1]?.body))).toEqual({ voice_receptionist_persona: 'Professional' });
});
it('a late preference acknowledgement preserves a newer provisioned number', async () => {
  let reply!: (response: Response) => void;
  saveVoice = () => new Promise(resolve => { reply = resolve; });
  provision = async () => Response.json({ success: true, number: '+14155550123' });
  render(<SettingsPage />);
  fireEvent.change(await screen.findByRole('combobox', { name: 'Voice Persona' }), { target: { value: 'Professional' } });
  await waitFor(() => expect(reply).toBeDefined());
  fireEvent.click(screen.getByRole('button', { name: 'Get Number' }));
  await waitFor(() => expect(screen.getByLabelText('Phone number')).toHaveValue('+14155550123'));
  await act(async () => reply(Response.json({ success: true })));
  expect(screen.getByLabelText('Phone number')).toHaveValue('+14155550123');
  expect(screen.getByRole('combobox', { name: 'Voice Persona' })).toHaveValue('Professional');
});
