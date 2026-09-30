import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, expect, it, vi } from 'vitest';
import type { ReactNode } from 'react';
import TasksPage from './page';

vi.mock('../../../app/components/AppShell', () => ({
  AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main>,
}));
afterEach(() => vi.unstubAllGlobals());

it('persists task creation, edits, and deletion across remounts', async () => {
  let records: { id: string; title: string; status: string }[] = [];
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => {
    if (url === '/api/v1/staff/tasks' && options?.method === 'POST') {
      const { title } = JSON.parse(String(options.body));
      records.push({ id: 'stored-task', title, status: 'pending' });
      return Response.json({ id: 'stored-task' });
    }
    if (url === '/api/v1/staff/tasks/stored-task' && options?.method === 'POST') {
      records[0].title = JSON.parse(String(options.body)).title;
      return Response.json({ success: true });
    }
    if (url === '/api/v1/staff/tasks/stored-task' && options?.method === 'DELETE') {
      records = [];
      return Response.json({ success: true });
    }
    return Response.json({ tasks: records });
  }));
  const user = userEvent.setup();
  let view = render(<TasksPage />);
  await user.click(screen.getByRole('button', { name: 'New Task' }));
  await user.type(screen.getByLabelText('Title'), 'A persisted task');
  await user.click(screen.getByRole('button', { name: 'Save' }));
  await screen.findByText('A persisted task');
  view.unmount();

  view = render(<TasksPage />);
  await user.click(await screen.findByText('A persisted task'));
  await user.click(screen.getByRole('button', { name: 'Edit' }));
  await user.clear(screen.getByLabelText('Title'));
  await user.type(screen.getByLabelText('Title'), 'Renamed persisted task');
  await user.click(screen.getByRole('button', { name: 'Save Changes' }));
  await within(screen.getByRole('list')).findByText('Renamed persisted task');
  view.unmount();

  view = render(<TasksPage />);
  await user.click(await screen.findByText('Renamed persisted task'));
  await user.click(screen.getByRole('button', { name: 'Delete' }));
  await screen.findByText('No tasks yet.');
  view.unmount();
  render(<TasksPage />);
  await screen.findByText('No tasks yet.');
  expect(screen.queryByText('Renamed persisted task')).toBeNull();
});

it('keeps an unsaved task open and reports a rejected write', async () => {
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => (
    options?.method === 'POST'
      ? Response.json({ error: 'unavailable' }, { status: 503 })
      : Response.json({ tasks: [] })
  )));
  const user = userEvent.setup();
  render(<TasksPage />);
  await user.click(screen.getByRole('button', { name: 'New Task' }));
  await user.type(screen.getByLabelText('Title'), 'Do not lose this draft');
  await user.click(screen.getByRole('button', { name: 'Save' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Could not save task. Please try again.');
  expect(screen.getByLabelText('Title')).toHaveValue('Do not lose this draft');
  expect(within(screen.getByRole('list')).queryByText('Do not lose this draft')).toBeNull();
});

it.each(['Edit', 'Delete'])('retains stored task data when %s is rejected', async (action) => {
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => (
    options?.method
      ? Response.json({ error: 'unavailable' }, { status: 503 })
      : Response.json({ tasks: [{ id: 'saved', title: 'Stored task', status: 'pending' }] })
  )));
  const user = userEvent.setup();
  render(<TasksPage />);
  await user.click(await screen.findByText('Stored task'));
  await user.click(screen.getByRole('button', { name: action }));
  if (action === 'Edit') {
    await user.clear(screen.getByLabelText('Title'));
    await user.type(screen.getByLabelText('Title'), 'Rejected edit');
    await user.click(screen.getByRole('button', { name: 'Save Changes' }));
  }
  expect(await screen.findByRole('alert')).toHaveTextContent(/Could not (save|delete) task/);
  expect(within(screen.getByRole('list')).getByText('Stored task')).toBeVisible();
  expect(within(screen.getByRole('list')).queryByText('Rejected edit')).toBeNull();
});
