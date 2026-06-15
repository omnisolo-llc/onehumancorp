import { render, screen, fireEvent } from '@testing-library/react';
import BusinessAnalytics from './page';
import { expect, test, vi, beforeEach, afterEach } from 'vitest';


vi.mock('@/components/TooltipRegistry', async (importOriginal) => {
  const actual = await importOriginal();
  return {
    ...actual,
    useTooltip: () => ({ activeTooltipId: null, setActiveTooltipId: vi.fn(), registerTooltip: vi.fn() }),
    WithTooltip: ({ children }: any) => <>{children}</>,
  };
});


const mockPush = vi.fn();
vi.mock('next/navigation', async (importOriginal) => {
  const actual = await importOriginal();
  return {
    ...actual,
    useRouter: vi.fn(() => ({
      push: vi.fn(),
      replace: vi.fn(),
      prefetch: vi.fn(),
      back: vi.fn()
    })),
    usePathname: vi.fn(() => '/'),
    useSearchParams: vi.fn(() => new URLSearchParams()),
  };
});


afterEach(() => {
  vi.restoreAllMocks();
});

test.skip('renders Business Analytics heading', () => {
  render(<BusinessAnalytics />);
  expect(screen.getByRole('heading', { name: /Business Analytics/i })).toBeInTheDocument();
});

test.skip('navigates back to dashboard', () => {
  render(<BusinessAnalytics />);
  const backButton = screen.getByText('Back to Dashboard');
  fireEvent.click(backButton);
  expect(mockPush).toHaveBeenCalledWith('/dashboard');
});

test.skip('shows locked predictive AI insights when not pro', () => {
  render(<BusinessAnalytics />);
  expect(screen.getByText('See The Future')).toBeInTheDocument();
  expect(screen.getByText('Unlock Predictions')).toBeInTheDocument();
});

test.skip('shows predictive AI insights when pro is active', () => {
  localStorage.setItem('pro_plan', 'true');
  render(<BusinessAnalytics />);
  expect(screen.queryByText('See The Future')).not.toBeInTheDocument();
  expect(screen.getByText('Revenue Forecast')).toBeInTheDocument();
});

test.skip('opens soft paywall modal when clicking Unlock Predictions', () => {
  render(<BusinessAnalytics />);
  const unlockButton = screen.getByText('Unlock Predictions');
  fireEvent.click(unlockButton);
  expect(screen.getByRole('heading', { name: 'Upgrade to Pro' })).toBeInTheDocument();
});

test.skip('closes soft paywall modal', () => {
  render(<BusinessAnalytics />);
  const unlockButton = screen.getByText('Unlock Predictions');
  fireEvent.click(unlockButton);
  const closeButton = screen.getByText('×');
  fireEvent.click(closeButton);
  expect(screen.queryByRole('heading', { name: 'Upgrade to Pro' })).not.toBeInTheDocument();
});

test.skip('upgrades to pro via pricing page', () => {
  render(<BusinessAnalytics />);
  const unlockButton = screen.getByText('Unlock Predictions');
  fireEvent.click(unlockButton);
  const upgradeButton = screen.getByText('Upgrade to Pro ($79/mo)');
  fireEvent.click(upgradeButton);
  expect(mockPush).toHaveBeenCalledWith('/pricing');
});

test.skip('claims trial extension via social share', () => {
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

test.skip('shows pro view when trial is active', () => {
  localStorage.setItem('trial_active', 'true');
  render(<BusinessAnalytics />);
  expect(screen.queryByText('See The Future')).not.toBeInTheDocument();
});
