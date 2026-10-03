import { act, fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, expect, it, vi } from 'vitest';
import Project from '../project-showcase/page';
import Lead from '../lead-magnet-generator/page';
import Referral from '../customer-referral-program/page';
import GroupBuy from '../group-buy-widget/page';
import TierList from '../viral-tier-list-generator/page';
import Roi from '../viral-roi-calculator/page';
import Powered from '../viral-powered-by-omnisolo-widget/page';
import Card from '../digital-business-card/page';
import Birthday from '../birthday-club/page';
import Poll from '../interactive-poll-generator/page';
import Product from '../viral-product-widget/page';
import Mystery from '../mystery-discount-generator/page';
import Fab from '../referral-fab-builder/page';
import Testimonial from '../testimonial-widget/page';
import Exit from '../exit-intent-builder/page';
import Event from '../event-rsvp-builder/page';
const plan = vi.hoisted(() => ({ hasPro: true }));
vi.mock('./useProPlan', () => ({ useProPlan: () => plan }));
vi.mock('./AppShell', () => ({ AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main> }));
vi.mock('./PoweredByOmniSolo', () => ({ PoweredByOmniSolo: () => <span>Powered by OmniSolo</span> }));
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn(), back: vi.fn() }) }));
beforeEach(() => { plan.hasPro = true; localStorage.clear(); Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: vi.fn().mockResolvedValue(undefined) } }); });
const copied = () => vi.mocked(navigator.clipboard.writeText).mock.calls.at(-1)![0];
it('project showcase copy follows current eligibility while retaining the configured project', async () => {
  const view = render(<Project />); fireEvent.click(screen.getByRole('checkbox'));
  await act(async () => fireEvent.click(screen.getByRole('button', { name: /Copy Share Link|Link Copied/ })));
  expect(new URL(copied()).searchParams.get('r')).toBe('1');
  plan.hasPro = false; view.rerender(<Project />);
  await act(async () => fireEvent.click(screen.getByRole('button', { name: /Copy Share Link|Link Copied/ })));
  expect(new URL(copied()).searchParams.get('r')).toBe('0');
  expect(screen.getByRole('checkbox')).not.toBeChecked();
});
it('lead magnet export restores branding after eligibility is lost and preserves the headline', async () => {
  const view = render(<Lead />); fireEvent.change(screen.getByLabelText('Headline'), { target: { value: 'Owner headline' } });
  fireEvent.click(screen.getByRole('checkbox')); await act(async () => fireEvent.click(screen.getByRole('button', { name: /Copy Code|Copied!/ })));
  expect(copied()).toContain('hideBranding=true');
  plan.hasPro = false; view.rerender(<Lead />); await act(async () => fireEvent.click(screen.getByRole('button', { name: /Copy Code|Copied!/ })));
  expect(copied()).toContain('hideBranding=false'); expect(copied()).toContain('Powered by OmniSolo');
  expect(screen.getByLabelText('Headline')).toHaveValue('Owner headline'); expect(screen.getByRole('checkbox')).not.toBeChecked();
});
it('customer referral embed copy cannot reuse an earlier Pro-only selection', async () => {
  const view = render(<Referral />); fireEvent.click(screen.getByRole('checkbox')); fireEvent.click(screen.getByRole('button', { name: /Generate Widget|Get Embed|Generate Embed/ }));
  await act(async () => fireEvent.click(screen.getByRole('button', { name: /Copy Code|Copied!/ })));
  expect(copied()).toContain('hide_branding=true');
  plan.hasPro = false; view.rerender(<Referral />); await act(async () => fireEvent.click(screen.getByRole('button', { name: /Copy Code|Copied!/ })));
  expect(copied()).toContain('hide_branding=false'); expect(copied()).toContain('Powered by OmniSolo');
  expect(screen.getByRole('checkbox')).not.toBeChecked();
});

const choices = [{ name: 'project', Page: Project }, { name: 'lead', Page: Lead }, { name: 'referral', Page: Referral }, { name: 'group-buy-widget', Page: GroupBuy }, { name: 'viral-tier-list-generator', Page: TierList }, { name: 'viral-roi-calculator', Page: Roi }, { name: 'viral-powered-by-omnisolo-widget', Page: Powered }, { name: 'digital-business-card', Page: Card }, { name: 'birthday-club', Page: Birthday }, { name: 'interactive-poll-generator', Page: Poll }, { name: 'viral-product-widget', Page: Product }, { name: 'mystery-discount-generator', Page: Mystery }, { name: 'referral-fab-builder', Page: Fab }, { name: 'testimonial-widget', Page: Testimonial }, { name: 'exit-intent-builder', Page: Exit }, { name: 'event-rsvp-builder', Page: Event }];
function choice() { return screen.queryAllByRole('checkbox', { hidden: true }).at(-1) ?? screen.queryByRole('switch') ?? screen.getByTestId('branding-toggle'); }
function selected(control: HTMLElement) { return control instanceof HTMLInputElement ? control.checked : control.getAttribute('aria-checked') === 'true' || control.classList.contains('bg-blue-600'); }
it.each(choices)('$name applies a saved selection only while the current plan permits it', ({ Page }) => {
  const view = render(<Page />); const control = choice(); if (!selected(control)) fireEvent.click(control);
  expect(selected(choice())).toBe(true);
  plan.hasPro = false; view.rerender(<Page />); expect(selected(choice())).toBe(false);
});
it('a cached Pro flag cannot override the actual plan in the powered widget', () => {
  plan.hasPro = false; localStorage.setItem('has_pro', 'true'); render(<Powered />);
  expect(screen.getByRole('checkbox')).not.toBeChecked(); fireEvent.click(screen.getByRole('checkbox'));
  expect(screen.getByText('Upgrade to Pro')).toBeVisible(); expect(localStorage.getItem('has_pro')).toBe('true');
});
it('a generated tier-list URL retires on lost eligibility while retaining the title', () => {
  const view = render(<TierList />); fireEvent.change(screen.getByLabelText('List Title'), { target: { value: 'Owner list' } }); fireEvent.click(screen.getByTestId('branding-toggle'));
  fireEvent.click(screen.getByRole('button', { name: 'Generate Share Link' })); expect((screen.getByTestId('generated-link') as HTMLInputElement).value).toContain('branding=false');
  plan.hasPro = false; view.rerender(<TierList />); expect(screen.queryByTestId('generated-link')).not.toBeInTheDocument(); expect(screen.getByLabelText('List Title')).toHaveValue('Owner list');
  fireEvent.click(screen.getByRole('button', { name: 'Generate Share Link' })); expect((screen.getByTestId('generated-link') as HTMLInputElement).value).toContain('branding=true');
});
it('a generated card URL cannot survive a lost Pro scope and be copied again', () => {
  const view = render(<Card />); fireEvent.change(screen.getByPlaceholderText('e.g. Jane Doe'), { target: { value: 'Owner Name' } }); fireEvent.change(screen.getByPlaceholderText('e.g. Founder & CEO'), { target: { value: 'Owner Title' } });
  fireEvent.click(screen.getByRole('checkbox')); fireEvent.click(screen.getByRole('button', { name: 'Generate Shareable Link' })); expect(screen.getByRole('button', { name: 'Copy' })).toBeVisible();
  plan.hasPro = false; view.rerender(<Card />); expect(screen.queryByRole('button', { name: 'Copy' })).not.toBeInTheDocument(); expect(screen.getByPlaceholderText('e.g. Jane Doe')).toHaveValue('Owner Name');
});
