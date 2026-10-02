import React from 'react';
import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import CustomerPage from './customer-referral-program/page';
import GeneratorPage from './share-to-unlock-generator/page';
import UnlockPage from './unlock/page';
const state = vi.hoisted(() => ({ search: new URLSearchParams(), hasPro: false }));
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn(), back: vi.fn() }), useSearchParams: () => state.search }));
vi.mock('./components/useProPlan', () => ({ useProPlan: () => ({ hasPro: state.hasPro }) }));
const deferred = () => { let resolve!: () => void; const promise = new Promise<void>(yes => { resolve = yes; }); return { promise, resolve }; };
const tenant = `owner & café / '雪'`;
beforeEach(() => {
  state.hasPro = false; state.search = new URLSearchParams();
  localStorage.clear(); localStorage.setItem('business_display_name', tenant);
  vi.spyOn(window, 'open').mockReturnValue(null);
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: vi.fn().mockResolvedValue(undefined) } });
});
afterEach(() => { cleanup(); vi.restoreAllMocks(); });
describe('customer referral preview', () => {
  it('does not invent a truncated customer link or enable an unconfigured referral action', () => {
    render(<CustomerPage />);
    for (const name of ['Copy', 'Share on WhatsApp', 'Share on X', 'Share on Facebook']) {
      const button = screen.getByRole('button', { name: new RegExp(`^${name}$`) });
      expect(button).toBeDisabled(); fireEvent.click(button);
    }
    expect(document.body.textContent).not.toContain('/ref/');
    expect(screen.getByText(/customer-specific referral link.*unavailable/i)).toBeVisible();
    expect(window.open).not.toHaveBeenCalled(); expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
  });
  it('awaits actual embed copying and never acknowledges a rejection', async () => {
    const pending = deferred(); vi.mocked(navigator.clipboard.writeText).mockReturnValueOnce(pending.promise);
    render(<CustomerPage />); fireEvent.click(screen.getByRole('button', { name: 'Generate Widget Embed' }));
    fireEvent.click(screen.getByRole('button', { name: 'Copy Code' }));
    expect(screen.queryByText('Copied!')).toBeNull(); expect(screen.getByRole('button', { name: 'Copying…' })).toBeDisabled();
    await act(async () => pending.resolve()); expect(screen.getByRole('button', { name: 'Copied!' })).toBeVisible();
    vi.mocked(navigator.clipboard.writeText).mockRejectedValueOnce(new Error('denied'));
    fireEvent.click(screen.getByRole('button', { name: 'Copied!' }));
    await act(async () => {}); expect(screen.queryByText('Copied!')).toBeNull(); expect(screen.getByRole('status')).toHaveTextContent('Copy failed');
  });
  it('preserves opaque configured tenant and the actual backend branding parameter in copied HTML', async () => {
    state.hasPro = true; render(<CustomerPage />);
    fireEvent.click(screen.getByRole('checkbox', { name: /Remove/ }));
    fireEvent.click(screen.getByRole('button', { name: 'Generate Widget Embed' })); fireEvent.click(screen.getByRole('button', { name: 'Copy Code' }));
    await act(async () => {});
    const html = vi.mocked(navigator.clipboard.writeText).mock.calls[0][0];
    const parsed = new DOMParser().parseFromString(html, 'text/html'); const frame = parsed.querySelector('iframe')!;
    const url = new URL(frame.getAttribute('src')!);
    expect(url.pathname).toBe('/api/v1/growth/customer-referral/embed'); expect(url.searchParams.get('tenant')).toBe(tenant);
    expect(url.searchParams.get('hide_branding')).toBe('true'); expect(url.searchParams.has('hideBranding')).toBe(false);
    expect(parsed.querySelector('script')).toBeNull(); expect(frame.attributes.getNamedItem('onload')).toBeNull();
  });
  it('does not resurrect copy success after closing and reopening the embed', async () => {
    const pending = deferred(); vi.mocked(navigator.clipboard.writeText).mockReturnValueOnce(pending.promise);
    render(<CustomerPage />); fireEvent.click(screen.getByRole('button', { name: 'Generate Widget Embed' })); fireEvent.click(screen.getByRole('button', { name: 'Copy Code' }));
    fireEvent.click(screen.getByRole('button', { name: 'Close modal' })); fireEvent.click(screen.getByRole('button', { name: 'Generate Widget Embed' }));
    await act(async () => pending.resolve()); expect(screen.queryByText('Copied!')).toBeNull();
  });
});
describe('configured share-to-unlock preview', () => {
  it('opens a correctly encoded preview intent and does not claim unlock or transmission', () => {
    render(<GeneratorPage />); fireEvent.click(screen.getByRole('button', { name: 'Share on WhatsApp' }));
    expect(window.open).toHaveBeenCalledTimes(1);
    const intent = new URL(vi.mocked(window.open).mock.calls[0][0] as string); expect(intent.hostname).toBe('wa.me');
    const text = intent.searchParams.get('text')!; const link = new URL(text.slice(text.lastIndexOf('http')));
    expect(link.pathname).toBe('/unlock'); expect(link.searchParams.get('tenant')).toBe(tenant); expect(link.searchParams.get('theme')).toBe('light');
    expect(text).not.toMatch(/I just unlocked/); expect(screen.getByRole('status')).toHaveTextContent(/verification.*unavailable/i);
    expect(screen.getByText('Locked')).toBeVisible();
  });
  it('copies the actual configured preview link only after platform completion', async () => {
    const pending = deferred(); vi.mocked(navigator.clipboard.writeText).mockReturnValueOnce(pending.promise);
    render(<GeneratorPage />); fireEvent.click(screen.getByRole('button', { name: 'Copy Link' }));
    expect(screen.queryByText('Copied!')).toBeNull();
    const url = new URL(vi.mocked(navigator.clipboard.writeText).mock.calls[0][0]); expect(url.searchParams.get('tenant')).toBe(tenant);
    await act(async () => pending.resolve()); expect(screen.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
  it('holds clipboard denial visibly and preserves the configured code', async () => {
    vi.mocked(navigator.clipboard.writeText).mockRejectedValueOnce(new Error('denied'));
    render(<GeneratorPage />); fireEvent.click(screen.getByRole('button', { name: 'Copy Link' })); await act(async () => {});
    expect(screen.queryByText('Copied!')).toBeNull(); expect(screen.getByText(/Copy failed/)).toBeVisible(); expect(screen.getByLabelText('Hidden Discount Code')).toHaveValue('SECRET20');
  });
  it('retires a pending copy when configuration changes and shows a blocked intent honestly', async () => {
    const pending = deferred(); vi.mocked(navigator.clipboard.writeText).mockReturnValueOnce(pending.promise);
    render(<GeneratorPage />); fireEvent.click(screen.getByRole('button', { name: 'Copy Link' }));
    fireEvent.change(screen.getByLabelText('Campaign Title'), { target: { value: 'Changed preview' } });
    await act(async () => pending.resolve()); expect(screen.queryByText('Copied!')).toBeNull();
    vi.mocked(window.open).mockImplementationOnce(() => { throw new Error('blocked'); });
    fireEvent.click(screen.getByRole('button', { name: 'Share preview on X' }));
    expect(screen.getByRole('status')).toHaveTextContent('could not be opened');
    expect(screen.getByText('Locked')).toBeVisible();
  });
  it('the actual destination preserves all fields and never promises sharing unlocks a reward', () => {
    state.search = new URLSearchParams({ tenant, title: 'A&B', reward: 'Configured offer', code: 'RAW&CODE', msg: 'Look here', theme: 'dark' });
    render(<UnlockPage />); fireEvent.click(screen.getByRole('button', { name: 'Share on X' }));
    const intent = new URL(vi.mocked(window.open).mock.calls[0][0] as string); const text = intent.searchParams.get('text')!; const url = new URL(text.slice(text.lastIndexOf('http')));
    for (const [key, value] of state.search) expect(url.searchParams.get(key)).toBe(value);
    expect(document.body.textContent).not.toMatch(/Share to reveal code|Unlock your special reward|Reward verified/);
    expect(screen.getByRole('status')).toHaveTextContent(/verification.*unavailable/i);
    const footer = screen.getByRole('link'); expect(new URL(footer.getAttribute('href')!, location.origin).searchParams.get('ref')).toBe(tenant);
  });
});
