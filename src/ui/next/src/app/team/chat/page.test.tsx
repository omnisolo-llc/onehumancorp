import { AppRouterContext } from "next/dist/shared/lib/app-router-context.shared-runtime";
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import TeamChatPage from './page';

const mockFetch = vi.fn();

beforeEach(() => {
  vi.clearAllMocks();
  global.fetch = mockFetch;

  Object.defineProperty(window, 'localStorage', {
    value: {
      getItem: vi.fn(() => 'test-token'),
      setItem: vi.fn(),
      removeItem: vi.fn(),
      clear: vi.fn(),
    },
    writable: true,
  });
});

test('shows a latency state while an AI action is being drafted', async () => {
  let resolveFetch: (value: Response) => void = () => {};
  mockFetch.mockReturnValue(
    new Promise<Response>((resolve) => {
      resolveFetch = resolve;
    }),
  );

  render(<AppRouterContext.Provider value={{} as any}><TeamChatPage /></AppRouterContext.Provider>);

  fireEvent.change(screen.getByTestId('team-chat-input'), {
    target: { value: 'Quote the sink repair' },
  });
  fireEvent.click(screen.getByTestId('team-chat-send'));

  expect(await screen.findByText('Working on your request...')).toBeInTheDocument();
  expect(screen.getByText('The team is still drafting the action.')).toBeInTheDocument();

  resolveFetch(
    new Response(
      JSON.stringify({
        agent: 'The Salesperson',
        description: 'Draft quote for Plumbing Fix',
      }),
      { status: 200, headers: { 'Content-Type': 'application/json' } },
    ),
  );

  await waitFor(() => {
    expect(screen.queryByText('Working on your request...')).not.toBeInTheDocument();
  });
  // Not waiting for it
});

test('renders an actionable error card when AI action execution fails', async () => {
  mockFetch.mockResolvedValue(
    new Response(JSON.stringify({ error: 'AI Budget exhausted' }), {
      status: 429,
      headers: { 'Content-Type': 'application/json' },
    }),
  );

  render(<AppRouterContext.Provider value={{} as any}><TeamChatPage /></AppRouterContext.Provider>);

  fireEvent.change(screen.getByTestId('team-chat-input'), {
    target: { value: 'Run the agent action' },
  });
  fireEvent.click(screen.getByTestId('team-chat-send'));

  expect(await screen.findByText('Action needs attention')).toBeInTheDocument();
  expect(screen.getByText('AI Budget exhausted')).toBeInTheDocument();
  expect(screen.getByText('Try again')).toBeInTheDocument();
});
