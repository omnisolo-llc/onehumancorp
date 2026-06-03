import React from 'react';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import Dashboard from './page';
import { expect, test, vi } from 'vitest';

// Mock fetch to prevent valid Undici errors regarding absolute URLs or missing globals
global.fetch = vi.fn(() => Promise.resolve({
  ok: true,
  json: () => Promise.resolve({})
})) as any;

test('renders dashboard with actionable feed and unified agentic intake', async () => {
  render(<TooltipProvider><Dashboard /></TooltipProvider>);

  await waitFor(() => {
    expect(screen.getAllByText("Business Analytics").length).toBeGreaterThan(0);
  });

  expect(screen.getByText("The Advisor Action Feed")).toBeDefined();
  expect(screen.getByText("Unified Agentic Intake")).toBeDefined();

  // Check for specific actionable buttons
  expect(screen.getByText("Yes, Draft Replies")).toBeDefined();
  const approveBtn = screen.getByText("Approve Quote");
  expect(approveBtn).toBeDefined();

  // Fire the click to simulate approval
  fireEvent.click(approveBtn);
  expect(screen.getByText("Quote Approved & Sent ✓")).toBeDefined();
});
