import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { TooltipProvider } from '../components/TooltipRegistry';
import Operations from './operations/page';
import Staff from './staff/page';
import PreOrder from './pre-order-widget/page';
import Demo from './interactive-demo/page';
import Poll from './interactive-poll-generator/page';
import TipJar from './tip-jar/page';

beforeEach(() => {
  localStorage.clear();
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => Response.json({ error: 'not_authenticated' }, { status: 401 })));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

it('sample appointments cannot send a client message', async () => {
  await act(async () => { render(<TooltipProvider><Operations /></TooltipProvider>); });
  const button = screen.getByRole('button', { name: 'Message Client' });
  expect(button).toBeDisabled();
  expect(button).toHaveAccessibleDescription(/unavailable for sample appointments/i);
  expect(screen.getByText(/sample schedule preview/i)).toBeInTheDocument();
});

it('sample staff shifts cannot request a live swap', async () => {
  await act(async () => { render(<TooltipProvider><Staff /></TooltipProvider>); });
  const button = screen.getByRole('button', { name: 'Request Swap' });
  expect(button).toBeDisabled();
  expect(button).toHaveAccessibleDescription(/sample schedule preview/i);
});

it('the waitlist preview describes its unavailable submission', async () => {
  await act(async () => { render(<TooltipProvider><PreOrder /></TooltipProvider>); });
  const button = screen.getByRole('button', { name: 'Join' });
  expect(button).toBeDisabled();
  expect(button).toHaveAccessibleDescription(/does not submit a waitlist entry/i);
});

it('both demo preview and exported markup disclose missing steps', async () => {
  await act(async () => { render(<Demo />); });
  const button = screen.getByRole('button', { name: 'Start Interactive Demo' });
  expect(button).toBeDisabled();
  expect(button).toHaveAccessibleDescription(/no interactive demo steps/i);
  const source = (screen.getByRole('textbox', { name: 'Embed code' }) as HTMLTextAreaElement).value;
  const parsed = new DOMParser().parseFromString(source, 'text/html');
  // DOMParser creates a detached document without a window in this runtime.
  // Inspect the actual exported attributes/text without a cross-window matcher.
  expect(parsed.querySelector('button')?.hasAttribute('disabled')).toBe(true);
  expect(parsed.getElementById('demo-content-unavailable')?.textContent).toMatch(/no interactive demo steps/i);
});

it('poll choices really change their exclusive local selection', async () => {
  await act(async () => { render(<TooltipProvider><Poll /></TooltipProvider>); });
  const chocolate = screen.getByRole('button', { name: 'Chocolate' });
  const vanilla = screen.getByRole('button', { name: 'Vanilla' });
  expect(chocolate).toHaveAttribute('aria-pressed', 'false');
  fireEvent.click(chocolate);
  expect(chocolate).toHaveAttribute('aria-pressed', 'true');
  fireEvent.click(vanilla);
  expect(chocolate).toHaveAttribute('aria-pressed', 'false');
  expect(vanilla).toHaveAttribute('aria-pressed', 'true');
  expect(screen.getByRole('button', { name: 'Vote Now' })).toBeDisabled();
});

it('changing an option clears the obsolete local choice', async () => {
  await act(async () => { render(<TooltipProvider><Poll /></TooltipProvider>); });
  fireEvent.click(screen.getByRole('button', { name: 'Chocolate' }));
  fireEvent.change(screen.getByDisplayValue('Chocolate'), { target: { value: 'Coffee' } });
  expect(screen.getByRole('button', { name: 'Coffee' })).toHaveAttribute('aria-pressed', 'false');
});

it('tip presets change local selection and clear a custom amount', async () => {
  await act(async () => { render(<TipJar />); });
  const five = screen.getByRole('button', { name: '$5' });
  const ten = screen.getByRole('button', { name: '$10' });
  fireEvent.click(five);
  expect(five).toHaveAttribute('aria-pressed', 'true');
  fireEvent.click(ten);
  expect(five).toHaveAttribute('aria-pressed', 'false');
  expect(ten).toHaveAttribute('aria-pressed', 'true');
  const custom = screen.getByPlaceholderText('Custom amount');
  fireEvent.change(custom, { target: { value: '12' } });
  expect(ten).toHaveAttribute('aria-pressed', 'false');
  fireEvent.click(five);
  expect(custom).toHaveValue(null);
});

it('the tip preview does not promise a payment', async () => {
  await act(async () => { render(<TipJar />); });
  const button = screen.getByRole('button', { name: 'Tip' });
  expect(button).toBeDisabled();
  expect(button).toHaveAccessibleDescription(/no payment is collected/i);
});
