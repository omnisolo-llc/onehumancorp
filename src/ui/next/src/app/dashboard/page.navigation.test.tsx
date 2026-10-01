import { act, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { TooltipProvider } from '../../components/TooltipRegistry';
import Dashboard from './page';

const navigation = vi.hoisted(() => ({ push: vi.fn() }));
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: navigation.push }),
  usePathname: () => '/dashboard',
  useSearchParams: () => new URLSearchParams(),
}));

beforeEach(() => {
  navigation.push.mockReset();
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ error: 'Unavailable in this component check' }, { status: 503 })));
});
afterEach(() => vi.unstubAllGlobals());

it('opens the real setup route when Launch Site is activated', async () => {
  await act(async () => { render(<TooltipProvider><Dashboard /></TooltipProvider>); });
  fireEvent.click(screen.getByRole('button', { name: 'Launch Site' }));
  expect(navigation.push).toHaveBeenCalledExactlyOnceWith('/onboarding');
});

it('offers a real proposal-editor link when dashboard data is unavailable', async () => {
  await act(async () => { render(<TooltipProvider><Dashboard /></TooltipProvider>); });
  expect(screen.getByRole('link', { name: /Proposals.*Proposal Draft/i })).toHaveAttribute('href', '/proposals/new');
});

it('opens and dismisses the actual quick-action links alongside shell actions', async () => {
  await act(async () => { render(<TooltipProvider><Dashboard /></TooltipProvider>); });
  expect(screen.getByRole('button', { name: 'New Product', exact: true })).toHaveAttribute('href', '/products/new');
  const toggle = screen.getByRole('button', { name: 'Quick Actions' });
  expect(screen.queryByRole('link', { name: '📦 New Product', exact: true })).toBeNull();
  fireEvent.click(toggle);
  expect(screen.getByRole('link', { name: '📦 New Product', exact: true })).toHaveAttribute('href', '/products/new');
  expect(screen.getByRole('link', { name: /Snap Receipt/ })).toHaveAttribute('href', '/dashboard/receipt');
  expect(screen.getByRole('link', { name: /Quick Charge/ })).toHaveAttribute('href', '/pos/terminal');
  fireEvent.click(toggle);
  expect(screen.queryByRole('link', { name: '📦 New Product', exact: true })).toBeNull();
});
