import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { beforeEach, expect, test, vi } from 'vitest';
import TriagePage from './page';
import mockRouter from 'next-router-mock';
import { TooltipProvider } from "../../components/TooltipRegistry";

vi.mock('next/navigation', () => ({
  useRouter: () => mockRouter,
  usePathname: () => mockRouter.pathname,
}));

global.fetch = vi.fn(() => Promise.resolve({
  ok: true,
  json: () => Promise.resolve([])
}));

test('renders triage empty state correctly', async () => {
  render(<TooltipProvider><TriagePage /></TooltipProvider>);
  expect(await screen.findByTestId('triage-feed-empty')).toBeInTheDocument();
});

test('handles fetching action cards', async () => {
  global.fetch = vi.fn(() => Promise.resolve({
    ok: true,
    json: () => Promise.resolve([{
      id: "card-1",
      context: "Need a quote",
      action_type: "Draft Quote",
      action_payload: "Quote for $150",
      created_at: new Date().toISOString()
    }])
  }));

  render(<TooltipProvider><TriagePage /></TooltipProvider>);

  expect(await screen.findByTestId('triage-card-card-1')).toBeInTheDocument();

  // Need to expand the card first to see the buttons
  const header = screen.getByTestId('triage-card-header-card-1');
  fireEvent.click(header);

  await waitFor(() => {
    // The exact string inside the div is: `AI Summary & Proposed Action: ${item.action_type}`
    expect(screen.getByText('AI Summary & Proposed Action: Draft Quote')).toBeInTheDocument();
  });

  const approveBtn = screen.getByTestId('triage-approve-card-1');
  const dismissBtn = screen.getByTestId('triage-dismiss-card-1');

  expect(approveBtn).toBeInTheDocument();
  expect(approveBtn).toHaveTextContent('Approve');
  expect(dismissBtn).toBeInTheDocument();
  expect(dismissBtn).toHaveTextContent('Reject');
});
