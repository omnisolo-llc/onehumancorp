import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { TooltipProvider } from '../components/TooltipRegistry';
import DigitalCard from './digital-business-card/page';
import CostDashboard from './cost-dashboard/page';
import Testimonial from './testimonial-widget/page';

beforeEach(() => {
  localStorage.clear();
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => Response.json({ error: 'not_authenticated' }, { status: 401 })));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });

it('cost details points to the actual rendered breakdown section', async () => {
  await act(async () => { render(<TooltipProvider><CostDashboard /></TooltipProvider>); });
  const link = screen.getByRole('link', { name: 'View Detailed Costs' });
  expect(link).toHaveAttribute('href', '#cost-breakdown-section');
  expect(document.querySelector(link.getAttribute('href')!)).toContainElement(screen.getByRole('heading', { name: 'Cost Breakdown' }));
});

it('card actions require supplied identity and generate a current share link', async () => {
  await act(async () => { render(<TooltipProvider><DigitalCard /></TooltipProvider>); });
  expect(screen.getByRole('button', { name: 'Save vCard' })).toBeDisabled();
  const share = screen.getByRole('button', { name: 'Create card link' });
  expect(share).toBeDisabled();
  fireEvent.change(screen.getByPlaceholderText('e.g. Jane Doe'), { target: { value: 'Alex Example' } });
  fireEvent.change(screen.getByPlaceholderText('e.g. Founder & CEO'), { target: { value: 'Designer' } });
  expect(share).toBeEnabled();
  fireEvent.click(share);
  const input = document.querySelector('input[readonly]') as HTMLInputElement;
  const data = JSON.parse(atob(new URL(input.value).searchParams.get('data')!.replace(/-/g, '+').replace(/_/g, '/')));
  expect(data.name).toBe('Alex Example');
  expect(data.title).toBe('Designer');
});

it('Save vCard creates a real escaped contact file and releases its object URL', async () => {
  let content!: Blob;
  const create = vi.fn((blob: Blob) => { content = blob; return 'blob:contact-download'; });
  const revoke = vi.fn();
  vi.stubGlobal('URL', class extends URL { static createObjectURL = create; static revokeObjectURL = revoke; });
  let downloaded = '';
  vi.spyOn(HTMLAnchorElement.prototype, 'click').mockImplementation(function (this: HTMLAnchorElement) { downloaded = this.download; expect(this.href).toBe('blob:contact-download'); });
  await act(async () => { render(<TooltipProvider><DigitalCard /></TooltipProvider>); });
  fireEvent.change(screen.getByPlaceholderText('e.g. Jane Doe'), { target: { value: 'Alex; Example' } });
  fireEvent.change(screen.getByPlaceholderText('e.g. Founder & CEO'), { target: { value: 'Designer' } });
  fireEvent.click(screen.getByRole('button', { name: 'Save vCard' }));
  expect(create).toHaveBeenCalledOnce();
  expect(downloaded).toMatch(/\.vcf$/);
  expect(content.type).toBe('text/vcard;charset=utf-8');
  const text = await new Promise<string>((resolve, reject) => { const reader = new FileReader(); reader.onload = () => resolve(String(reader.result)); reader.onerror = reject; reader.readAsText(content); });
  expect(text).toContain('BEGIN:VCARD\r\nVERSION:3.0\r\n');
  expect(text).toContain('FN:Alex\\; Example\r\n');
  expect(text).toContain('TITLE:Designer\r\n');
  expect(text).toContain('END:VCARD\r\n');
  await act(async () => { await new Promise(resolve => setTimeout(resolve, 0)); });
  expect(revoke).toHaveBeenCalledWith('blob:contact-download');
});

it('testimonial navigation points to the actual dashboard', async () => {
  await act(async () => { render(<Testimonial />); });
  expect(screen.getByRole('link', { name: 'Back to Dashboard' })).toHaveAttribute('href', '/dashboard');
});
