import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { TooltipProvider } from '../components/TooltipRegistry';
import DigitalCard from './digital-business-card/page';
import EmailSignature from './email-signature-generator/page';
import ExitIntent from './exit-intent-builder/page';
import InteractiveDemo from './interactive-demo/page';
import EmbedBuilder from './embed-builder/page';
import TipJar from './tip-jar/page';
import Poll from './interactive-poll-generator/page';
import Quote from './interactive-quote-generator/page';
import Referral from './referral-widget/page';
import Affiliate from './affiliate-badge-builder/page';
import TeamGrowth from './components/GrowthReferralWidget';
import Testimonial from './testimonial-widget/page';

function deferred() {
  let resolve!: () => void;
  let reject!: (reason: unknown) => void;
  const promise = new Promise<void>((yes, no) => { resolve = yes; reject = no; });
  // Preserve an assertion failure instead of an unhandled test-platform promise.
  void promise.catch(() => {});
  return { promise, resolve, reject };
}
const cases = [
  { name: 'testimonial', Page: Testimonial, button: /^Copy Code$/, prepare: () => fireEvent.click(screen.getByRole('button', { name: 'Get Widget Code' })) },
  { name: 'team embed', Page: TeamGrowth, button: /^Copy Embed Code$/, prepare: () => {} },
  { name: 'embed builder', Page: EmbedBuilder, button: /^Copy Code$/, prepare: () => {} },
  { name: 'tip jar', Page: TipJar, button: /^Copy Code$/, prepare: () => fireEvent.click(screen.getByRole('button', { name: 'Get Widget Code' })) },
  { name: 'poll', Page: Poll, button: /^Copy Code$/, prepare: () => fireEvent.click(screen.getByRole('button', { name: 'Generate Embed Code' })) },
  { name: 'quote', Page: Quote, button: /^Copy Embed Code$/, prepare: () => {} },
  { name: 'referral', Page: Referral, button: /^Copy Code$/, prepare: async () => {
    const open = screen.getByRole('button', { name: 'Get Embed Code' });
    await waitFor(() => expect(open).toBeEnabled());
    fireEvent.click(open);
  } },
  { name: 'affiliate', Page: Affiliate, button: /^Copy Code$/, prepare: () => fireEvent.click(screen.getByRole('button', { name: 'Get Embed Code' })) },
  { name: 'business card', Page: DigitalCard, button: /^Copy$/, prepare: () => {
    fireEvent.change(screen.getByPlaceholderText('e.g. Jane Doe'), { target: { value: 'Alex Example' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Founder & CEO'), { target: { value: 'Designer' } });
    fireEvent.click(screen.getByRole('button', { name: 'Generate Shareable Link' }));
  } },
  { name: 'signature', Page: EmailSignature, button: /Copy Signature HTML/, prepare: () => {} },
  { name: 'exit intent', Page: ExitIntent, button: /Copy to Clipboard/, prepare: () => {} },
  { name: 'interactive demo', Page: InteractiveDemo, button: /^Copy$/, prepare: () => {} },
];

beforeEach(() => {
  localStorage.clear();
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => Response.json({ error: 'not_authenticated' }, { status: 401 })));
  vi.stubGlobal('ClipboardItem', class { constructor(readonly data: Record<string, Blob>) {} });
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

function verifyQuoteFixture(name: string) {
  if (name === 'quote') vi.mocked(fetch).mockImplementation(async url => String(url).endsWith('/session-identity')
    ? Response.json({ userId: 'quote-owner', tenantId: 'quote-tenant', expiresAt: Date.now() + 60_000 })
    : Response.json({ error: 'not_authenticated' }, { status: 401 }));
  if (name === 'referral') vi.mocked(fetch).mockImplementation(async url => {
    if (String(url) === '/api/v1/auth/session-identity') return Response.json({ userId: 'referral-owner', tenantId: 'referral-tenant', expiresAt: Date.now() + 60_000 });
    if (String(url) === '/api/v1/billing/my-plan') return Response.json({ current_plan: 'Free' });
    throw new Error(`Unexpected referral fixture request: ${url}`);
  });
}

describe('clipboard feedback follows the actual platform outcome', () => {
  for (const { name, Page, button, prepare } of cases) {
    it(`${name} stays pending until copying succeeds`, async () => {
      const completion = deferred();
      const writeText = vi.fn<(text: string) => Promise<void>>(() => completion.promise);
      const write = vi.fn<(items: ClipboardItem[]) => Promise<void>>(() => completion.promise);
      Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText, write } });
      verifyQuoteFixture(name);
      await act(async () => { render(<TooltipProvider><Page /></TooltipProvider>); });
      await prepare();
      const copy = screen.getByRole('button', { name: button });
      fireEvent.click(copy);
      expect(copy).toBeDisabled();
      const copyFeedback = ['team embed', 'referral'].includes(name) ? within(copy.parentElement!) : screen;
      expect(copyFeedback.getByRole('status')).toHaveTextContent(/copying/i);
      expect(screen.queryByText(/^Copied!?$/)).not.toBeInTheDocument();
      await act(async () => { completion.resolve(); });
      await waitFor(() => expect(copyFeedback.getByRole('status')).toHaveTextContent(/copied/i));
      expect(copy).toBeEnabled();
      expect(writeText.mock.calls.length + write.mock.calls.length).toBe(1);
    });

    it(`${name} reports denial without claiming copied`, async () => {
      const completion = deferred();
      const writeText = vi.fn<(text: string) => Promise<void>>(() => completion.promise);
      const write = vi.fn<(items: ClipboardItem[]) => Promise<void>>(() => completion.promise);
      Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText, write } });
      verifyQuoteFixture(name);
      await act(async () => { render(<TooltipProvider><Page /></TooltipProvider>); });
      await prepare();
      fireEvent.click(screen.getByRole('button', { name: button }));
      await act(async () => { completion.reject(new DOMException('Denied', 'NotAllowedError')); });
      expect(screen.getByRole('alert')).toHaveTextContent(/copy failed/i);
      expect(screen.queryByText(/^Copied!?$/)).not.toBeInTheDocument();
      if (name === 'signature') expect(writeText).not.toHaveBeenCalled();
    });
  }
});

it('a pending acknowledgement cannot mark newer demo code as copied', async () => {
  const completion = deferred();
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: () => completion.promise } });
  await act(async () => { render(<InteractiveDemo />); });
  const copy = screen.getByRole('button', { name: 'Copy' });
  fireEvent.click(copy);
  expect(copy).toBeDisabled();
  expect(screen.getByRole('status')).toHaveTextContent(/copying/i);
  fireEvent.change(screen.getByDisplayValue('My Interactive Demo'), { target: { value: 'New demo' } });
  await act(async () => { completion.resolve(); });
  expect(screen.queryByText(/^Copied!?$/)).not.toBeInTheDocument();
  expect((document.querySelector('textarea[readonly]') as HTMLTextAreaElement).value).toContain('New demo');
});

it('signature fallback only acknowledges the completed plain-HTML copy', async () => {
  const completion = deferred();
  const write = vi.fn(async () => { throw new DOMException('Rich HTML unsupported', 'NotSupportedError'); });
  const writeText = vi.fn<(text: string) => Promise<void>>(() => completion.promise);
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { write, writeText } });
  await act(async () => { render(<EmailSignature />); });
  const copy = screen.getByRole('button', { name: /Copy Signature HTML/ });
  await act(async () => { fireEvent.click(copy); });
  expect(writeText).toHaveBeenCalledOnce();
  expect(writeText.mock.calls[0][0]).toContain('Jane Doe');
  expect(copy).toBeDisabled();
  expect(screen.getByRole('status')).toHaveTextContent(/copying/i);
  await act(async () => { completion.resolve(); });
  expect(screen.getByRole('status')).toHaveTextContent(/copied/i);
});

it('an older rich-copy rejection cannot overwrite a newer copied signature', async () => {
  const old = deferred();
  const write = vi.fn().mockImplementationOnce(() => old.promise).mockResolvedValue(undefined);
  const writeText = vi.fn().mockResolvedValue(undefined);
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { write, writeText } });
  await act(async () => { render(<EmailSignature />); });
  fireEvent.click(screen.getByRole('button', { name: /Copy Signature HTML/ }));
  fireEvent.change(screen.getByDisplayValue('Jane Doe'), { target: { value: 'New Name' } });
  await act(async () => { fireEvent.click(screen.getByRole('button', { name: /Copy Signature HTML/ })); });
  expect(screen.getByRole('status')).toHaveTextContent(/copied/i);
  await act(async () => { old.reject(new DOMException('Unsupported', 'NotSupportedError')); });
  expect(writeText).not.toHaveBeenCalled();
  expect(write).toHaveBeenCalledTimes(2);
  expect(screen.getByRole('status')).toHaveTextContent(/copied/i);
});
