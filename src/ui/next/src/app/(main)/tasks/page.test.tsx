import { act, render, screen, within } from '@testing-library/react';
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

it('retains a task until its finite DELETE acknowledgement body completes', async () => {
  let finish!: () => void;
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => options?.method === 'DELETE'
    ? new Response(new ReadableStream({ start(controller) { finish = () => { controller.enqueue(new TextEncoder().encode('{"success":true}')); controller.close(); }; } }), { status: 200, headers: { 'content-type': 'application/json' } })
    : Response.json({ tasks: [{ id: 'saved', title: 'Held task', status: 'pending' }] })));
  const user = userEvent.setup(); render(<TasksPage />);
  await user.click(await screen.findByText('Held task'));
  await user.click(screen.getByRole('button', { name: 'Delete' }));
  expect(within(screen.getByRole('list')).getByText('Held task')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Delete' })).toBeDisabled();
  await act(async () => finish());
  await screen.findByText('No tasks yet.');
});
it.each([{ success: false }, {}, { success: true, error: 'not committed' }])('retains the task on an invalid HTTP200 DELETE receipt %#', async receipt => {
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => options?.method === 'DELETE'
    ? Response.json(receipt)
    : Response.json({ tasks: [{ id: 'saved', title: 'Held task', status: 'pending' }] })));
  const user = userEvent.setup(); render(<TasksPage />);
  await user.click(await screen.findByText('Held task'));
  await user.click(screen.getByRole('button', { name: 'Delete' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Could not delete task');
  expect(within(screen.getByRole('list')).getByText('Held task')).toBeVisible();
});
it('does not apply an edit after a contradictory HTTP200 acknowledgement', async () => {
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => options?.method === 'POST'
    ? Response.json({ success: false })
    : Response.json({ tasks: [{ id: 'saved', title: 'Original title', status: 'pending' }] })));
  const user = userEvent.setup(); render(<TasksPage />);
  await user.click(await screen.findByText('Original title'));await user.click(screen.getByRole('button', { name: 'Edit' }));
  await user.clear(screen.getByLabelText('Title'));await user.type(screen.getByLabelText('Title'), 'Uncommitted title');
  await user.click(screen.getByRole('button', { name: 'Save Changes' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Could not save task');
  expect(within(screen.getByRole('list')).getByText('Original title')).toBeVisible();
  expect(screen.getByLabelText('Title')).toHaveValue('Uncommitted title');
});
it.each([false, 0, 'yes'])('does not treat an ID with a non-true creation success field as a saved task %#', async success => {
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => options?.method === 'POST'
    ? Response.json({ id: 'not-committed', success })
    : Response.json({ tasks: [] })));
  const user = userEvent.setup();render(<TasksPage />);
  await user.click(screen.getByRole('button', { name: 'New Task' }));await user.type(screen.getByLabelText('Title'), 'Keep draft');
  await user.click(screen.getByRole('button', { name: 'Save' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Could not save task');
  expect(screen.getByLabelText('Title')).toHaveValue('Keep draft');
  expect(within(screen.getByRole('list')).queryByText('Keep draft')).toBeNull();
});
it.each(['create', 'edit', 'delete'])('does not treat an accepted202 %s request as committed', async operation => {
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => options?.method
    ? Response.json({ id: 'pending', success: true }, { status: 202 })
    : Response.json({ tasks: [{ id: 'saved', title: 'Stored task', status: 'pending' }] })));
  const user = userEvent.setup(); render(<TasksPage />);
  if (operation === 'create') {
    await user.click(screen.getByRole('button', { name: 'New Task' }));
    await user.type(screen.getByLabelText('Title'), 'Pending task');
    await user.click(screen.getByRole('button', { name: 'Save' }));
  } else {
    await user.click(await screen.findByText('Stored task'));
    await user.click(screen.getByRole('button', { name: operation === 'edit' ? 'Edit' : 'Delete' }));
    if (operation === 'edit') {
      await user.clear(screen.getByLabelText('Title')); await user.type(screen.getByLabelText('Title'), 'Pending task');
      await user.click(screen.getByRole('button', { name: 'Save Changes' }));
    }
  }
  expect(await screen.findByRole('alert')).toHaveTextContent(/Could not (save|delete) task/);
  expect(within(screen.getByRole('list')).getByText('Stored task')).toBeVisible();
  expect(within(screen.getByRole('list')).queryByText('Pending task')).toBeNull();
});
