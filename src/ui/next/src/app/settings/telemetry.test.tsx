import { act, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import type { ReactNode } from 'react';
import SettingsPage from './page';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main> }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: ReactNode }) => <>{children}</> }));
let current: boolean;
let canChange: boolean;
let operatorEnforced: boolean;
let blockReason: string | null;
let save: () => Promise<Response>;
beforeEach(() => {
  current = false; canChange = true; operatorEnforced = false; blockReason = null; save = async () => Response.json({ success: true });
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => {
    if (url === '/api/v1/settings/telemetry') {
      if (options?.method === 'POST') {
        const response = await save();
        if (response.ok && (await response.clone().json()).success === true) current = JSON.parse(String(options.body)).product_telemetry_enabled;
        return response;
      }
      return Response.json({ product_telemetry_enabled: current, preference_enabled: current, effective_enabled: operatorEnforced || current, operator_enforced: operatorEnforced, can_change: canChange, change_block_reason: blockReason });
    }
    return Response.json({});
  }));
  vi.spyOn(console, 'error').mockImplementation(() => {});
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
it.each([403, 500, 200])('reverts telemetry opt-in and reports an explicit HTTP %s failure', async status => {
  save = async () => Response.json({ success: false }, { status });
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
  await userEvent.click(toggle);
  expect(await screen.findByRole('alert')).toHaveTextContent('Telemetry change could not be confirmed');
  expect(toggle).not.toBeChecked();
});
it('serializes saves and reports success only after the server acknowledges', async () => {
  let resolve!: (response: Response) => void;
  save = () => new Promise<Response>(done => { resolve = done; });
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
  await userEvent.click(toggle);
  expect(toggle).toBeDisabled();
  expect(screen.getByText('Saving telemetry preference…')).toHaveAttribute('role', 'status');
  await act(async () => resolve(Response.json({ success: true })));
  await waitFor(() => expect(toggle).toBeEnabled());
  expect(toggle).toBeChecked();
  expect(screen.getByText('Telemetry preference saved.')).toHaveAttribute('role', 'status');
});
it('does not treat unavailable telemetry settings as a verified opt-out', async () => {
  vi.mocked(fetch).mockImplementation(async (url: string | URL | Request) => String(url).includes('/settings/telemetry') ? new Response('{}', { status: 500 }) : Response.json({}));
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
  expect(toggle).toBeDisabled();
  expect(screen.getByRole('alert')).toHaveTextContent('Telemetry preference is unavailable');
});
it('re-reads the actual setting after an uncertain save instead of asserting rollback', async () => {
  save = async () => { current = true; throw new Error('Response lost after applying'); };
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
  await userEvent.click(toggle);
  await screen.findByRole('alert');
  expect(toggle).toBeChecked();
  expect(vi.mocked(fetch).mock.calls.filter(([url, options]) => url === '/api/v1/settings/telemetry' && options?.method !== 'POST')).toHaveLength(2);
});

it.each(['hosted_global_control_unavailable', 'admin_required', 'persistent_storage_unavailable'])('does not offer writes when server policy reports %s', async reason => {
  canChange = false; blockReason = reason;
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
  expect(toggle).toBeDisabled();
  await userEvent.click(toggle);
  expect(vi.mocked(fetch).mock.calls.filter(([, options]) => options?.method === 'POST')).toHaveLength(0);
});
it('shows operator-enforced effective telemetry as on even when the saved preference is off', async () => {
  canChange = false; operatorEnforced = true; blockReason = 'operator_enforced';
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
  expect(toggle).toBeChecked(); expect(toggle).toBeDisabled();
  expect(screen.getByText(/Effective telemetry: On/)).toBeVisible();
  expect(screen.getByText(/enabled by server configuration/)).toBeVisible();
});
it('never infers write permission from a legacy preference-only response', async () => {
  vi.mocked(fetch).mockImplementation(async (url: string | URL | Request) => String(url).includes('/settings/telemetry') ? Response.json({ product_telemetry_enabled: false }) : Response.json({}));
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
  expect(toggle).toBeDisabled();
  expect(screen.getByRole('alert')).toHaveTextContent('Telemetry preference is unavailable');
});
it('does not report saved when the acknowledged preference differs from the requested value', async () => {
  vi.mocked(fetch).mockImplementation(async (url: string | URL | Request, options?: RequestInit) => String(url).includes('/settings/telemetry') ? Response.json(options?.method === 'POST' ? { success: true } : { product_telemetry_enabled: false, preference_enabled: false, effective_enabled: false, operator_enforced: false, can_change: true }) : Response.json({}));
  render(<SettingsPage />);
  const toggle = await screen.findByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
  await userEvent.click(toggle);
  expect(await screen.findByRole('alert')).toHaveTextContent('could not be confirmed');
  expect(toggle).not.toBeChecked();
  expect(screen.queryByText('Telemetry preference saved.')).not.toBeInTheDocument();
});
