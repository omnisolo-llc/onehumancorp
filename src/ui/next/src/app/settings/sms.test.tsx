import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import type { ReactNode } from 'react';
import SettingsPage from './page';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main> }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: ReactNode }) => children }));
const identity = { userId: 'owner-a', tenantId: 'tenant-a', expiresAt: 4_102_444_800_000 };
const binding = { user_id: identity.userId, organization_id: identity.tenantId };
const preferences = { urgent_booking: false, failed_payment: false, new_order: false };
const challenge = { challenge_id: '00000000-0000-4000-8000-000000000001', phone: '+14155550123', state: 'accepted', expires_at: 4_102_444_800 };
const snapshot = (verified = false) => ({ success: true, ...binding, status: verified ? 'verified' : 'unverified', phone: verified ? challenge.phone : null, verification_id: verified ? challenge.challenge_id : null, preferences, challenge: null, provider_configured: true });
let readSms: () => Response | Promise<Response>;
let sendSms: () => Response | Promise<Response>;
let confirmSms: () => Response | Promise<Response>;
let saveSms: () => Response | Promise<Response>;
beforeEach(() => {
  vi.spyOn(crypto, 'randomUUID').mockReturnValue(challenge.challenge_id as `${string}-${string}-${string}-${string}-${string}`);
  readSms = () => Response.json(snapshot());
  sendSms = () => Response.json({ success: true, ...binding, status: 'provider_accepted', ...challenge });
  confirmSms = () => Response.json({ success: false, error: 'invalid_or_expired_code' });
  saveSms = () => Response.json({ success: false });
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => {
    if (url === '/api/v1/auth/session-identity') return Response.json(identity);
    if (url === '/api/v1/settings/sms-preferences') return options?.method === 'POST' ? saveSms() : readSms();
    if (url === '/api/v1/settings/sms-verify') return sendSms();
    if (url === '/api/v1/settings/sms-confirm') return confirmSms();
    return Response.json({});
  }));
  vi.spyOn(console, 'error').mockImplementation(() => {});
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
async function requestCode() {
  const field = await screen.findByLabelText('Mobile Number');
  await waitFor(() => expect(field).not.toBeDisabled());
  fireEvent.change(field, { target: { value: challenge.phone } });
  fireEvent.click(screen.getByRole('button', { name: 'Verify Number' }));
  await screen.findByLabelText('Verification code');
}
it('never verifies a phone from an HTTP200 negative OTP receipt', async () => {
  render(<SettingsPage />); await requestCode();
  fireEvent.change(screen.getByLabelText('Verification code'), { target: { value: '000000' } });
  fireEvent.click(screen.getByRole('button', { name: 'Confirm OTP' }));
  await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/settings/sms-confirm')).toBe(true));
  expect(screen.queryByText(/number verified/i)).toBeNull();
  expect(screen.getByRole('checkbox', { name: 'Urgent Bookings' })).toBeDisabled();
});
it('loads verified settings from the server rather than resetting them to false', async () => {
  readSms = () => Response.json({ ...snapshot(true), preferences: { ...preferences, urgent_booking: true } });
  render(<SettingsPage />);
  await waitFor(() => expect(screen.getByRole('checkbox', { name: 'Urgent Bookings' })).toBeChecked());
  expect(screen.getByLabelText('Mobile Number')).toHaveValue(challenge.phone);
});
it('treats unavailable settings as unknown and disables sending', async () => {
  readSms = () => Response.json({ success: false, error: 'storage_unavailable' }, { status: 503 });
  render(<SettingsPage />);
  expect(await screen.findByText(/SMS settings could not be verified/i)).toBeVisible();
  expect(screen.getByRole('button', { name: 'Verify Number' })).toBeDisabled();
});
it('does not claim an accepted SMS from an HTTP200 false receipt', async () => {
  sendSms = () => Response.json({ success: false, status: 'provider_accepted', ...challenge, ...binding });
  render(<SettingsPage />);
  const phone = await screen.findByLabelText('Mobile Number');
  await waitFor(() => expect(phone).not.toBeDisabled());
  fireEvent.change(phone, { target: { value: challenge.phone } });
  fireEvent.click(screen.getByRole('button', { name: 'Verify Number' }));
  expect(await screen.findByText(/SMS request could not be confirmed/i)).toBeVisible();
  expect(screen.queryByLabelText('Verification code')).toBeNull();
});
it('does not retain a preference update rejected with HTTP200', async () => {
  readSms = () => Response.json(snapshot(true)); render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Urgent Bookings' });
  await waitFor(() => expect(toggle).not.toBeDisabled()); fireEvent.click(toggle);
  expect(await screen.findByText(/SMS preference change could not be confirmed/i)).toBeVisible();
  expect(toggle).not.toBeChecked();
});
it('ignores a late private read after identity changes', async () => {
  let done!: (r: Response) => void; readSms = () => new Promise(resolve => { done = resolve; });
  render(<SettingsPage />); await waitFor(() => expect(done).toBeDefined());
  act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
  await act(async () => done(Response.json(snapshot(true))));
  expect(screen.getByLabelText('Mobile Number')).toHaveValue('');
  expect(screen.getByRole('checkbox', { name: 'Urgent Bookings' })).toBeDisabled();
});
it.each(['foreign-owner', 'foreign-tenant', 'contradictory'])('rejects a %s verification receipt', async kind => {
  confirmSms = () => Response.json({ ...snapshot(true), ...(kind === 'foreign-owner' ? { user_id: 'another-owner' } : kind === 'foreign-tenant' ? { organization_id: 'another-tenant' } : { error: 'denied' }) });
  render(<SettingsPage />); await requestCode();
  fireEvent.change(screen.getByLabelText('Verification code'), { target: { value: '123456' } }); fireEvent.click(screen.getByRole('button', { name: 'Confirm OTP' }));
  expect(await screen.findByText(/code could not be verified/i)).toBeVisible();
  expect(screen.getByRole('checkbox', { name: 'Urgent Bookings' })).toBeDisabled();
});
it('does not repeat an uncertain SMS send while checking its saved status', async () => {
  sendSms = async () => { throw new Error('response lost'); };
  render(<SettingsPage />); const phone = await screen.findByLabelText('Mobile Number'); await waitFor(() => expect(phone).not.toBeDisabled());
  fireEvent.change(phone, { target: { value: challenge.phone } }); fireEvent.click(screen.getByRole('button', { name: 'Verify Number' }));
  expect(await screen.findByText(/SMS request could not be confirmed/i)).toBeVisible();
  const send = screen.getByRole('button', { name: 'Verify Number' }); expect(send).toBeDisabled();fireEvent.click(send);
  expect(vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/settings/sms-verify')).toHaveLength(1);
  readSms = () => Response.json({ ...snapshot(), challenge: { ...challenge, state: 'unknown' } });
  fireEvent.click(screen.getByRole('button', { name: 'Check saved SMS status' }));
  await screen.findByText('Saved SMS status checked.'); expect(send).toBeDisabled();
});
it('ignores an SMS acceptance reply received after session invalidation', async () => {
  let done!: (r: Response) => void; sendSms = () => new Promise(resolve => { done = resolve; });
  render(<SettingsPage />); const phone = await screen.findByLabelText('Mobile Number'); await waitFor(() => expect(phone).not.toBeDisabled());
  fireEvent.change(phone, { target: { value: challenge.phone } }); fireEvent.click(screen.getByRole('button', { name: 'Verify Number' }));
  await waitFor(() => expect(done).toBeDefined());act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
  await act(async () => done(Response.json({ success: true, ...binding, ...challenge, status: 'provider_accepted' })));
  expect(phone).toHaveValue(''); expect(screen.queryByLabelText('Verification code')).toBeNull();
});
it('requires a current matching saved verification receipt in preference writes', async () => {
  readSms = () => Response.json(snapshot(true)); saveSms = () => Response.json({ ...snapshot(true), preferences: { ...preferences, new_order: true } });
  render(<SettingsPage />);const toggle = await screen.findByRole('checkbox', { name: 'New Orders' });await waitFor(() => expect(toggle).not.toBeDisabled());fireEvent.click(toggle);
  await screen.findByText('SMS preferences saved.');expect(toggle).toBeChecked();
  const call = vi.mocked(fetch).mock.calls.find(([url, options]) => url === '/api/v1/settings/sms-preferences' && options?.method === 'POST');
  expect(JSON.parse(String(call?.[1]?.body))).toEqual({ phone: challenge.phone, verification_id: challenge.challenge_id, ...preferences, new_order: true });
  expect(call?.[1]?.headers).toEqual(expect.objectContaining({ 'x-ohc-expected-user': identity.userId, 'x-ohc-expected-tenant': identity.tenantId }));
});
it('explains that saving New Orders does not enable automatic order SMS', async () => {
  readSms = () => Response.json(snapshot(true));
  saveSms = () => Response.json({ ...snapshot(true), preferences: { ...preferences, new_order: true } });
  const view = render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'New Orders' });
  await waitFor(() => expect(toggle).not.toBeDisabled());
  expect(toggle).toHaveAccessibleDescription('You can save this preference. Automatic new-order SMS is currently unavailable until orders have a confirmed, saved receipt.');
  fireEvent.click(toggle);
  await screen.findByText('SMS preferences saved.');
  expect(toggle).toBeChecked();
  expect(screen.getByText(/Automatic new-order SMS is currently unavailable/)).toBeVisible();
  view.unmount();
  readSms = () => Response.json({ ...snapshot(true), preferences: { ...preferences, new_order: true } });
  render(<SettingsPage />);
  await waitFor(() => expect(screen.getByRole('checkbox', { name: 'New Orders' })).toBeChecked());
  expect(screen.getByRole('checkbox', { name: 'New Orders' })).toHaveAccessibleDescription(/Automatic new-order SMS is currently unavailable/);
});
