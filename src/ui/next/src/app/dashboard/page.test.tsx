import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor } from '@testing-library/react';
import Dashboard from './page';
import { expect, test, vi } from 'vitest';
import userEvent from '@testing-library/user-event';

// Mock fetch to prevent valid Undici errors regarding absolute URLs or missing globals
global.fetch = vi.fn((url: string) => {
  if (url && typeof url === 'string' && (url.includes('proposals') || url.includes('/api/agents/proposals'))) {
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve([
        {
          id: 'prop-1',
          department: 'Advisory',
          title: 'Test Proposal',
          actionLabel: 'Yes, draft it',
          status: 'pending',
          expandedContent: 'Drafted Email Content',
          expandedActionLabel: 'Approve & Send'
        }
      ])
    });
  }
  return Promise.resolve({
  ok: true,
  json: () => Promise.resolve([])
  });
}) as any;

test('renders dashboard with actionable feed', async () => {
  render(<TooltipProvider><Dashboard /></TooltipProvider>);

  await waitFor(() => {
    expect(screen.getAllByText("Business Analytics").length).toBeGreaterThan(0);
  });

  expect(screen.getByText("Operations Map")).toBeDefined();
  expect(screen.getByText(/Action Required/)).toBeDefined();
  expect(screen.getByText("Recent Orders")).toBeDefined();
  expect(screen.getByText("Inbox Activity")).toBeDefined();
});

test('renders Unified Agent Feed and handles expansion', async () => {
  render(<TooltipProvider><Dashboard /></TooltipProvider>);

  await waitFor(
    () => expect(screen.getAllByText("Unified Agent Feed").length).toBeGreaterThan(0),
    { timeout: 4000 }
  );

  expect(screen.getAllByText("Test Proposal").length).toBeGreaterThan(0);

  const actionButton = screen.getByText("Yes, draft it");
  await userEvent.click(actionButton);

  expect(screen.getAllByText("Drafted Email Content").length).toBeGreaterThan(0);
  expect(screen.getAllByText("Approve & Send").length).toBeGreaterThan(0);
});
