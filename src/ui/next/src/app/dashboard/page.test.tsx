import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen } from '@testing-library/react';
import Dashboard from './page';
import { expect, test } from 'vitest';

test('renders dashboard with actionable feed', () => {
  render(<TooltipProvider><Dashboard /></TooltipProvider>);
  expect(screen.getByText("Business Analytics")).toBeDefined();
  expect(screen.getByText(/Action Required/)).toBeDefined();
  expect(screen.getByText("Complete Stripe Setup")).toBeDefined();
  expect(screen.getByText("Weekly Insights")).toBeDefined();
  expect(screen.getByText("AI Business Advisory")).toBeDefined();
});
