import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen } from '@testing-library/react';
import Dashboard from './page';
import { expect, test } from 'vitest';

test('renders dashboard with actionable feed', () => {
  render(<TooltipProvider><Dashboard /></TooltipProvider>);
  expect(screen.getByText("Today's Pulse")).toBeDefined();
  expect(screen.getByText("Action Required")).toBeDefined();
  expect(screen.getByText(/2 Custom Cake Orders to Review/)).toBeDefined();
  expect(screen.getByText(/Approve Instagram post/)).toBeDefined();
  expect(screen.getByText(/Weekly Insights Available/)).toBeDefined();
});
