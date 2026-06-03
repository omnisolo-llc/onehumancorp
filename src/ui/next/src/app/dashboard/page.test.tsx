import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor } from '@testing-library/react';
import Dashboard from './page';
import { expect, test, vi } from 'vitest';

// Mock fetch to prevent valid Undici errors regarding absolute URLs or missing globals
global.fetch = vi.fn(() => Promise.resolve({
  ok: true,
  json: () => Promise.resolve({})
})) as any;

test('renders dashboard with actionable feed', async () => {
  render(<TooltipProvider><Dashboard /></TooltipProvider>);

  await waitFor(() => {
    expect(screen.getAllByText("Business Analytics").length).toBeGreaterThan(0);
  });

<<<<<<< HEAD
  expect(screen.getByText("Operations Map")).toBeDefined();
  expect(screen.getByText(/Action Required/)).toBeDefined();
  expect(screen.getByText("Recent Orders")).toBeDefined();
  expect(screen.getByText("Inbox Activity")).toBeDefined();
=======
  expect(screen.getByText(/Action Required/)).toBeDefined();
  expect(screen.getByText("Complete Stripe Setup")).toBeDefined();
  expect(screen.getByText("Weekly Insights")).toBeDefined();
  expect(screen.getByText("AI Business Advisory")).toBeDefined();
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
});
