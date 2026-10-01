import '@testing-library/jest-dom';
import { act, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import MarketplacePage from './page';
import PublishPage from './publish/page';
const navigation = vi.hoisted(() => ({ push: vi.fn() }));
vi.mock('next/navigation', () => ({ useRouter: () => navigation }));
const agent = { id: 'registry-agent', name: 'Actual registry descriptor', description: 'A definition link', author: 'Registry author', version: '1.0.0', endpoint: 'https://registry.example.test/definition' };
beforeEach(() => {
  navigation.push.mockReset();
  vi.stubGlobal('fetch', vi.fn(async (_url, options) => Response.json(options?.method === 'POST' ? { id: 'fabricated-success' } : [agent])));
});
afterEach(() => vi.unstubAllGlobals());
it('never presents a local button toggle as a recorded agent installation', async () => {
  render(<MarketplacePage />);
  const install = await screen.findByRole('button', { name: 'Install Agent' });
  expect(install).toBeDisabled();
  fireEvent.click(install);
  expect(screen.queryByRole('button', { name: 'Installed' })).toBeNull();
  expect(screen.queryByText(/Successfully installed/)).toBeNull();
  expect(screen.getByText(/Installation is unavailable/)).toBeVisible();
  expect(vi.mocked(fetch).mock.calls.some(([, options]) => options?.method === 'POST')).toBe(false);
});
it('keeps full-agent form values without sending fields the registry cannot persist', async () => {
  render(<PublishPage />);
  const values = { 'Agent Name': 'Private draft', Description: 'Reviewed purpose', Role: 'Writer', 'System Prompt': 'Private instructions' };
  for (const [name, value] of Object.entries(values)) fireEvent.change(screen.getByLabelText(name), { target: { value } });
  const button = screen.getByRole('button', { name: 'Publish to Marketplace' });
  await act(async () => fireEvent.submit(button.closest('form')!));
  expect(fetch).not.toHaveBeenCalled();
  expect(navigation.push).not.toHaveBeenCalled();
  for (const [name, value] of Object.entries(values)) expect(screen.getByLabelText(name)).toHaveValue(value);
  expect(button).toBeDisabled();
  expect(screen.getByRole('status')).toHaveTextContent(/Full-agent publication is unavailable/);
});
