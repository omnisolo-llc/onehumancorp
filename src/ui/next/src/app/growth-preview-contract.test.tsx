import React from 'react';
import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Coupon from './viral-coupon-unlock/page';
import Streak from './viral-streak-widget/page';
import Jobs from './viral-job-board-generator/page';
import Goal from './viral-goal-tracker/page';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('./components/useProPlan', () => ({ useProPlan: () => ({ hasPro: false }) }));
const tenant = `owner & café / '雪' " onload="alert(1)`;
beforeEach(() => {
  localStorage.clear(); localStorage.setItem('business_display_name', tenant);
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: vi.fn().mockResolvedValue(undefined) } });
  vi.spyOn(window, 'open').mockReturnValue(null);
});
afterEach(() => { cleanup(); vi.restoreAllMocks(); });
it('holds coupon sharing and copying until a real publication exists', () => {
  render(<Coupon />);
  for (const name of ['Copy Link', 'Share on X', 'Share on WhatsApp']) {
    const button = screen.getByRole('button', { name }); expect(button).toBeDisabled(); fireEvent.click(button);
  }
  expect(document.body.textContent).not.toContain('/unlock/my-business');
  expect(navigator.clipboard.writeText).not.toHaveBeenCalled(); expect(window.open).not.toHaveBeenCalled();
});
it('keeps coupon configuration without fabricating progress or eligibility', () => {
  render(<Coupon />); fireEvent.change(screen.getByLabelText('Shares Required to Unlock'), { target: { value: '5' } });
  expect(screen.getByLabelText('Hidden Coupon Code')).toHaveValue('WELCOME20');
  expect(screen.getByText('Configured target: 5 shares')).toBeVisible();
  expect(screen.getByText(/Share progress is unavailable/)).toBeVisible();
  expect(document.body.textContent).not.toMatch(/1 \/ 5|Unlock this exclusive coupon by sharing/);
});
it('does not offer an embed for the unmounted streak endpoint', () => {
  render(<Streak />); const button = screen.getByRole('button', { name: 'Get Embed Code' });
  expect(button).toBeDisabled(); fireEvent.click(button);
  expect(screen.queryByRole('button', { name: 'Copy Code' })).toBeNull();
  expect(document.body.textContent).not.toContain('/viral-streak/embed');
});
it('keeps streak configuration but reports no recorded visits or claim', () => {
  render(<Streak />); expect(screen.getByRole('button', { name: "Claim Today's Streak" })).toBeDisabled();
  expect(screen.getByLabelText('Reward')).toHaveValue('Free Coffee');
  expect(screen.getByText(/Visit tracking and reward claims are unavailable/)).toBeVisible();
  expect(document.body.textContent).not.toContain('✓');
});
it('job theme buttons reflect actual exclusive choices and rendered colors', () => {
  render(<Jobs />); const light = screen.getByRole('button', { name: 'Light' }); const dark = screen.getByRole('button', { name: 'Dark' });
  const preview = screen.getByRole('heading', { name: 'We are hiring!' }).parentElement!;
  expect(light).toHaveAttribute('aria-pressed', 'true'); expect(dark).toHaveAttribute('aria-pressed', 'false');
  expect(preview).toHaveStyle({ backgroundColor: '#fff' });
  fireEvent.click(dark); expect(dark).toHaveAttribute('aria-pressed', 'true'); expect(light).toHaveAttribute('aria-pressed', 'false'); expect(preview).toHaveStyle({ backgroundColor: '#111827' });
  fireEvent.click(light); expect(light).toHaveAttribute('aria-pressed', 'true'); expect(dark).toHaveAttribute('aria-pressed', 'false'); expect(preview).toHaveStyle({ backgroundColor: '#fff' });
});
it('does not invent a published job board or referral destination', () => {
  render(<Jobs />);
  for (const name of ['Copy Link', 'Refer a Friend']) expect(screen.getByRole('button', { name })).toBeDisabled();
  expect(document.body.textContent).not.toContain('/jobs/'); expect(screen.getByText(/example roles/i)).toBeVisible();
});
it('does not promise an unsupported hiring reward', () => {
  render(<Jobs />); expect(document.body.textContent).not.toMatch(/\$500|Refer a friend and get/);
  expect(screen.getByText(/Referral attribution and rewards are unavailable/)).toBeVisible();
});
it('goal preview keeps its configured target without claiming referral progress', () => {
  render(<Goal />); expect(screen.getByRole('button', { name: 'Share to reach goal' })).toBeDisabled();
  expect(screen.getByText('10 target')).toBeVisible(); expect(screen.getByText(/Referral progress is unavailable/)).toBeVisible();
  expect(document.body.textContent).not.toMatch(/0 referrals in preview|Unlock:/);
});
it('the actual goal embed preserves opaque values and its mounted query contract', async () => {
  render(<Goal />); fireEvent.click(screen.getByRole('button', { name: 'Copy Embed Code' })); await act(async () => {});
  const html = vi.mocked(navigator.clipboard.writeText).mock.calls[0][0]; const parsed = new DOMParser().parseFromString(html, 'text/html');
  const frame = parsed.querySelector('iframe')!; const url = new URL(frame.getAttribute('src')!);
  expect(url.pathname).toBe('/api/v1/growth/viral-goal-tracker'); expect(url.searchParams.get('tenant')).toBe(tenant);
  expect(url.searchParams.get('target')).toBe('10'); expect(url.searchParams.get('hideBranding')).toBe('false'); expect(url.searchParams.has('hide_branding')).toBe(false);
  expect(frame.getAttribute('onload')).toBeNull(); expect(parsed.querySelector('script')).toBeNull();
});
it('goal copy waits for the platform and retires stale completion and rejection', async () => {
  let finish!: () => void; vi.mocked(navigator.clipboard.writeText).mockReturnValueOnce(new Promise<void>(resolve => { finish = resolve; }));
  render(<Goal />); fireEvent.click(screen.getByRole('button', { name: 'Copy Embed Code' }));
  expect(screen.queryByText('Copied to Clipboard!')).toBeNull(); expect(screen.getByRole('button', { name: 'Copying…' })).toBeDisabled();
  fireEvent.change(screen.getByLabelText('Reward Name'), { target: { value: 'New configured offer' } }); await act(async () => finish());
  expect(screen.queryByText('Copied to Clipboard!')).toBeNull(); vi.mocked(navigator.clipboard.writeText).mockRejectedValueOnce(new Error('denied'));
  fireEvent.click(screen.getByRole('button', { name: 'Copy Embed Code' })); await act(async () => {});
  expect(screen.getByRole('status')).toHaveTextContent(/Copy failed/); expect(screen.getByLabelText('Reward Name')).toHaveValue('New configured offer');
});
