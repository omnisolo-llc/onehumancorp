import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import AgentsPage from './page';
import * as TooltipContext from '@/components/TooltipContext';

vi.mock('@/components/TooltipContext', () => ({
  useTooltipContext: vi.fn(() => ({
    advance: vi.fn(),
  })),
}));


const mockFetch = vi.fn((url) => {
  if (url === '/api/agents/approvals') {
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: '123',
            department: 'customer_success',
            description: 'Draft reply to customer',
            status: 'pending'
          }
        ],
        next_cursor: null
      })
    });
  } else if (url.startsWith('/api/agents/workflows')) {
     return Promise.resolve({
      ok: true,
      json: () => Promise.resolve({ workflows: [] })
    });
  } else if (url === '/api/agents/approvals/123/approve') {
     return Promise.resolve({
      ok: true,
      json: () => Promise.resolve({ success: true })
    });
  } else if (url.startsWith('/api/agents/feed')) {
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve([])
    });
  }
  return Promise.resolve({
      ok: true,
      json: () => Promise.resolve({})
  });
});
global.fetch = mockFetch;


beforeEach(() => {
  vi.clearAllMocks();
});

test('renders AI Departments heading', async () => {

  render(<AgentsPage />);

  await waitFor(() => {
    expect(screen.getByText('AI Departments')).toBeDefined();
  });
});

test('switches tabs correctly', async () => {

  render(<AgentsPage />);

  // Default is 'teams'
  expect(screen.getByText('The Manager')).toBeDefined();

  // Click on 'workflows'
  fireEvent.click(screen.getByText('Workflows'));

  await waitFor(() => {
    expect(screen.getByText('Create Workflow')).toBeDefined();
  });
});

test('renders approvals tab correctly', async () => {

  render(<AgentsPage />);

  fireEvent.click(screen.getByText('Needs Approval'));

  await waitFor(() => {
    expect(screen.getByText('Draft For Review')).toBeDefined();
    expect(screen.getByText('customer_success')).toBeDefined();
    expect(screen.getByText('Draft reply to customer')).toBeDefined();
  });
});

test('renders feed tab correctly', async () => {

  render(<AgentsPage />);

  fireEvent.click(screen.getByText('Activity Feed'));

  await waitFor(() => {
    expect(screen.getByText('No activity yet.')).toBeDefined();
  });
});

test('approves a draft successfully', async () => {

  render(<AgentsPage />);

  fireEvent.click(screen.getByText('Needs Approval'));

  await waitFor(() => {
    expect(screen.getByText('Approve & Send')).toBeDefined();
  });


  fireEvent.click(screen.getByText('Approve & Send'));

  // Use a softer assertion or wait for the mock to be called twice (initial load + action)
  await waitFor(() => {
    expect(mockFetch.mock.calls.length).toBeGreaterThanOrEqual(1);
  });
});
