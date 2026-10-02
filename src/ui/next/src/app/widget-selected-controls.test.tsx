import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import DigitalBusinessCard from './digital-business-card/page';
import QrCodeGenerator from './qr-code-generator/page';
import EmbedBuilder from './embed-builder/page';
import InteractiveQuote from './interactive-quote-generator/page';
import ReferralWidget from './referral-widget/page';
import AiWorkspace from './ai-workspace/page';
import Affiliate from './affiliate-badge-builder/page';
import { TooltipProvider } from '../components/TooltipRegistry';

beforeEach(() => {
  localStorage.clear();
  // These editor interactions require no account or provider authority.
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () =>
    Response.json({ error: 'not_authenticated' }, { status: 401 })));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

describe('editor choices expose their actual exclusive selection', () => {
  for (const { name, Page, selected, alternate } of [
    { name: 'affiliate theme', Page: Affiliate, selected: 'Dark', alternate: 'Light' },
    { name: 'business card color', Page: DigitalBusinessCard, selected: 'Select color #4F46E5', alternate: 'Select color #000000' },
    { name: 'QR color', Page: QrCodeGenerator, selected: 'Select color #111827', alternate: 'Select color #4f46e5' },
    { name: 'embed type', Page: EmbedBuilder, selected: 'intake', alternate: 'booking' },
    { name: 'embed theme', Page: EmbedBuilder, selected: 'Light', alternate: 'Dark' },
    { name: 'quote theme', Page: InteractiveQuote, selected: 'Light', alternate: 'Dark' },
    { name: 'referral theme', Page: ReferralWidget, selected: 'Light', alternate: 'Dark' },
    { name: 'workspace tab', Page: AiWorkspace, selected: 'overview', alternate: 'tasks' },
  ]) {
    it(`${name} changes selection and can restore the original choice`, async () => {
      await act(async () => { render(<TooltipProvider><Page /></TooltipProvider>); });
      const original = screen.getByRole('button', { name: selected });
      const other = screen.getByRole('button', { name: alternate });
      expect(original).toHaveAttribute('aria-pressed', 'true');
      expect(other).toHaveAttribute('aria-pressed', 'false');
      fireEvent.click(other);
      expect(original).toHaveAttribute('aria-pressed', 'false');
      expect(other).toHaveAttribute('aria-pressed', 'true');
      fireEvent.click(original);
      expect(original).toHaveAttribute('aria-pressed', 'true');
      expect(other).toHaveAttribute('aria-pressed', 'false');
    });
  }
});

it('the focus timer only offers Reset when it has a change to undo', async () => {
  await act(async () => { render(<TooltipProvider><AiWorkspace /></TooltipProvider>); });
  const reset = screen.getByRole('button', { name: 'Reset' });
  expect(reset).toBeDisabled();
  fireEvent.click(screen.getByRole('button', { name: 'Start Focus' }));
  expect(reset).toBeEnabled();
  fireEvent.click(reset);
  expect(screen.getByRole('button', { name: 'Start Focus' })).toBeInTheDocument();
  expect(reset).toBeDisabled();
});
