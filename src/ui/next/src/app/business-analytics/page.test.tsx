import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import BusinessAnalytics from './page';
import { expect, test, vi, beforeEach, afterEach } from 'vitest';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
  usePathname: () => '/business-analytics',
}));

vi.mock('../../components/TooltipRegistry', () => ({
  TooltipProvider: ({ children }: any) => children,
  WithTooltip: ({ children }: any) => children,
}));

beforeEach(() => {
  localStorage.clear();
  vi.clearAllMocks();
  // Mock window.open for trial
  global.window.open = vi.fn();
  global.fetch = vi.fn(() => Promise.resolve({ ok: true, json: () => Promise.resolve({ total_sales: 100, pending_orders: 10, active_customers: 5 }) })) as any;
  // Mock alert
  global.alert = vi.fn();
});

afterEach(() => {
  vi.restoreAllMocks();
});

test('renders Business Analytics heading', () => {
  render(<BusinessAnalytics />);
  expect(screen.getByRole('heading', { name: /Business Analytics/i })).toBeInTheDocument();
});

test('navigates back to dashboard', () => {
  render(<BusinessAnalytics />);
  const backButton = screen.getByText('Back to Dashboard').closest('a');
  expect(backButton).toHaveAttribute('href', '/dashboard');
});

test('shows locked predictive AI insights when not pro', () => {
  render(<BusinessAnalytics />);
  expect(screen.getByText('See The Future')).toBeInTheDocument();
  expect(screen.getByText('Unlock Predictions')).toBeInTheDocument();
});

test('shows predictive AI insights when pro is active', () => {
  localStorage.setItem('pro_plan', 'true');
  render(<BusinessAnalytics />);
  expect(screen.queryByText('See The Future')).not.toBeInTheDocument();
  expect(screen.getByText('Predictive Revenue Forecast')).toBeInTheDocument();
  expect(screen.getByText('Cohort Retention Analysis')).toBeInTheDocument();
});

test('opens soft paywall modal when clicking Unlock Predictions', () => {
  render(<BusinessAnalytics />);
  const unlockButton = screen.getByText('Unlock Predictions');
  fireEvent.click(unlockButton);
  expect(screen.getByRole('heading', { name: 'Upgrade to Pro' })).toBeInTheDocument();
});

test('closes soft paywall modal', () => {
  render(<BusinessAnalytics />);
  const unlockButton = screen.getByText('Unlock Predictions');
  fireEvent.click(unlockButton);
  const closeButton = screen.getByText('×');
  fireEvent.click(closeButton);
  expect(screen.queryByRole('heading', { name: 'Upgrade to Pro' })).not.toBeInTheDocument();
});

test('upgrades to pro via pricing page', () => {
  render(<BusinessAnalytics />);
  const unlockButton = screen.getByText('Unlock Predictions');
  fireEvent.click(unlockButton);
  const upgradeButton = screen.getByText('Upgrade to Pro ($79/mo)');
  fireEvent.click(upgradeButton);
  expect(mockPush).toHaveBeenCalledWith('/pricing');
});

test('claims trial extension via social share', () => {
  render(<BusinessAnalytics />);
  const unlockButton = screen.getByText('Unlock Predictions');
  fireEvent.click(unlockButton);
  const shareButton = screen.getByText(/Share on X to unlock 7 Days Free/i);
  fireEvent.click(shareButton);

  expect(global.window.open).toHaveBeenCalled();
  expect(localStorage.getItem('trial_active')).toBe('true');
  expect(screen.getByRole('status')).toHaveTextContent('7-day Pro Trial activated');
  expect(screen.queryByRole('heading', { name: 'Upgrade to Pro' })).not.toBeInTheDocument();
});

test('shows pro view when trial is active', () => {
  localStorage.setItem('trial_active', 'true');
  render(<BusinessAnalytics />);
  expect(screen.queryByText('See The Future')).not.toBeInTheDocument();
});
