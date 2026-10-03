import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import type { ReactNode } from 'react';
import ScalingPage from './page';

vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main> }));
const fetchMock = vi.fn<typeof fetch>();
beforeEach(() => { fetchMock.mockReset(); vi.stubGlobal('fetch', fetchMock); });
const deploy = () => {
  render(<ScalingPage />);
  fireEvent.change(screen.getByPlaceholderText('e.g. Analyze dataset'), { target: { value: 'Retain this task' } });
  fireEvent.click(screen.getByRole('button', { name: 'Deploy Scalable Multi-Agent Fleet' }));
};

test('preserves unavailable runtime errors without counting them as agent output', async () => {
  fetchMock.mockResolvedValue(Response.json({ error: 'Agent runtime is not configured; no work was dispatched' }, { status: 503 }));
  deploy();
  expect(await screen.findByRole('alert')).toHaveTextContent('Agent runtime is not configured; no work was dispatched');
  expect(screen.queryByRole('heading', { name: /Results/ })).not.toBeInTheDocument();
  expect(screen.queryByText('Agent 1:')).not.toBeInTheDocument();
  expect(screen.getByPlaceholderText('e.g. Analyze dataset')).toHaveValue('Retain this task');
  expect(screen.getByRole('button', { name: 'Deploy Scalable Multi-Agent Fleet' })).toBeEnabled();
});

test.each([{}, { outputs: [] }, { outputs: [''] }, { outputs: ['one', 2] }, { outputs: ['one'], status: 'queued' }, { outputs: ['one'], success: false }, { outputs: ['one'], error: 'failed' }])('rejects unconfirmed output %#', async payload => {
  fetchMock.mockResolvedValue(Response.json(payload));
  deploy();
  expect(await screen.findByRole('alert')).toHaveTextContent('Backend returned no confirmed agent outputs');
  expect(screen.queryByRole('heading', { name: /Results/ })).not.toBeInTheDocument();
});

test('renders genuine completed outputs and replaces them with an error on a later failure', async () => {
  fetchMock.mockResolvedValueOnce(Response.json({ outputs: ['First completed analysis', 'Second completed analysis'] }));
  deploy();
  expect(await screen.findByRole('heading', { name: 'Results (2 outputs)' })).toBeVisible();
  expect(screen.getByText('First completed analysis')).toBeVisible();
  fetchMock.mockResolvedValueOnce(Response.json({ error: 'Runtime unavailable' }, { status: 503 }));
  fireEvent.click(screen.getByRole('button', { name: 'Deploy Scalable Multi-Agent Fleet' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Runtime unavailable');
  expect(screen.queryByText('First completed analysis')).not.toBeInTheDocument();
});
