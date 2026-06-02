import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import AgentsPage from './page';
import * as TooltipContext from '@/components/TooltipContext';

vi.mock('@/components/TooltipContext', () => ({
  useTooltipContext: vi.fn(() => ({
    advance: vi.fn(),
  })),
}));

const mockFetch = vi.fn();

beforeEach(() => {
  vi.clearAllMocks();
  global.fetch = mockFetch;
  mockFetch.mockImplementation((url: string) => {
    if (url.includes('/api/agents/workflows')) {
      return Promise.resolve({ ok: true, json: async () => ({ workflows: [] }) });
    }
    if (url.includes('/api/agents/feed')) {
      return Promise.resolve({ ok: true, json: async () => ({ feed: [], next_cursor: null }) });
    }
    if (url.includes('/api/agents/approvals')) {
      return Promise.resolve({ ok: true, json: async () => ({ pending_approvals: [], next_cursor: null }) });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
  });
});

test('renders AI Departments heading', async () => {
<<<<<<< HEAD
=======
  mockFetch.mockResolvedValue({
    ok: true,
    json: async () => ({ pending_approvals: [], next_cursor: null, workflows: [] }),
  });

>>>>>>> ca73a8e3 (feat(ui): enhance onboarding styles and fix test mocks)
  render(<AgentsPage />);

  await waitFor(() => {
    expect(screen.getByText('AI Departments')).toBeDefined();
  });
});

test('switches tabs correctly', async () => {
<<<<<<< HEAD
=======
  mockFetch.mockResolvedValue({
    ok: true,
    json: async () => ({ pending_approvals: [], next_cursor: null, workflows: [] }),
  });

>>>>>>> ca73a8e3 (feat(ui): enhance onboarding styles and fix test mocks)
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
<<<<<<< HEAD
  mockFetch.mockImplementation((url: string) => {
    if (url.includes('/api/agents/workflows')) return Promise.resolve({ ok: true, json: async () => ({ workflows: [] }) });
    if (url.includes('/api/agents/feed')) return Promise.resolve({ ok: true, json: async () => ({ feed: [], next_cursor: null }) });
    if (url.includes('/api/agents/approvals')) {
      return Promise.resolve({
        ok: true,
        json: async () => ({
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
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
=======
  mockFetch.mockResolvedValue({
    ok: true,
    json: async () => ({
      workflows: [],
      pending_approvals: [
        {
          id: '123',
          department: 'customer_success',
          description: 'Draft reply to customer',
          status: 'pending'
        }
      ],
      next_cursor: null
    }),
>>>>>>> ca73a8e3 (feat(ui): enhance onboarding styles and fix test mocks)
  });

  render(<AgentsPage />);

  fireEvent.click(screen.getByText('Needs Approval'));

  await waitFor(() => {
    expect(screen.getByText('Draft For Review')).toBeDefined();
    expect(screen.getByText('customer_success')).toBeDefined();
    expect(screen.getByText('Draft reply to customer')).toBeDefined();
  });
});

test('renders feed tab correctly', async () => {
<<<<<<< HEAD
=======
  mockFetch.mockResolvedValue({
    ok: true,
    json: async () => ({ pending_approvals: [], next_cursor: null, workflows: [] }),
  });

>>>>>>> ca73a8e3 (feat(ui): enhance onboarding styles and fix test mocks)
  render(<AgentsPage />);

  fireEvent.click(screen.getByText('Activity Feed'));

  await waitFor(() => {
    expect(screen.getByText('No activity yet.')).toBeDefined();
  });
});

test('approves a draft successfully', async () => {
<<<<<<< HEAD
  mockFetch.mockImplementation((url: string) => {
    if (url.includes('/api/agents/workflows')) return Promise.resolve({ ok: true, json: async () => ({ workflows: [] }) });
    if (url.includes('/api/agents/feed')) return Promise.resolve({ ok: true, json: async () => ({ feed: [], next_cursor: null }) });
    if (url.includes('/api/agents/approvals/123')) return Promise.resolve({ ok: true, json: async () => ({ success: true }) });
    if (url.includes('/api/agents/approvals')) {
      return Promise.resolve({
        ok: true,
        json: async () => ({
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
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
=======
  mockFetch.mockResolvedValue({
    ok: true,
    json: async () => ({
      workflows: [],
      pending_approvals: [
        {
          id: '123',
          department: 'customer_success',
          description: 'Draft reply to customer',
          status: 'pending'
        }
      ],
      next_cursor: null
    }),
>>>>>>> ca73a8e3 (feat(ui): enhance onboarding styles and fix test mocks)
  });

  render(<AgentsPage />);

  fireEvent.click(screen.getByText('Needs Approval'));

  await waitFor(() => {
    expect(screen.getByText('Approve & Send')).toBeDefined();
  });

  fireEvent.click(screen.getByText('Approve & Send'));

  // Wait for the mock to have been called for the approval API
  await waitFor(() => {
    expect(mockFetch).toHaveBeenCalledWith(expect.stringContaining('/api/agents/approvals/123'), expect.objectContaining({ method: 'POST' }));
  });
});
