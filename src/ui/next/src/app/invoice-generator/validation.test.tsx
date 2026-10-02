import { fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import InvoiceGeneratorPage from './page';
let alert: ReturnType<typeof vi.spyOn>;
beforeEach(() => { localStorage.clear(); alert = vi.spyOn(window, 'alert').mockImplementation(() => {}); });
afterEach(() => vi.restoreAllMocks());
const submit = () => fireEvent.click(screen.getByRole('button', { name: 'Generate Shareable Invoice' }));
function fill(client: string, details: string, amount: string) {
  fireEvent.change(screen.getByLabelText('Client Name'), { target: { value: client } });
  fireEvent.change(screen.getByLabelText('Project Details'), { target: { value: details } });
  fireEvent.change(screen.getByLabelText('Amount ($)'), { target: { value: amount } });
}
it.each(['', '   '])('shows accessible inline errors for blank invoice details: %j', value => {
  render(<InvoiceGeneratorPage />); fill(value, value, ''); submit();
  expect(alert).not.toHaveBeenCalled(); expect(screen.getByRole('alert')).toHaveTextContent('Review the highlighted invoice details.');
  expect(screen.getByLabelText('Client Name')).toHaveAttribute('aria-invalid', 'true');
  expect(screen.getByLabelText('Project Details')).toHaveAttribute('aria-invalid', 'true');
  expect(screen.getByLabelText('Amount ($)')).toHaveAttribute('aria-invalid', 'true');
  expect(screen.getByLabelText('Client Name')).toHaveAttribute('aria-describedby', 'client-name-error');
  expect(screen.queryByRole('link', { name: 'Preview Invoice' })).not.toBeInTheDocument();
});
it.each(['0', '-1', '-0.01', '1e309', 'NaN'])('rejects a nonpositive or nonfinite amount without generating an invoice: %s', amount => {
  render(<InvoiceGeneratorPage />); fill('Owner client', 'Reviewed project', amount); submit();
  expect(screen.getByText('Enter a valid amount greater than zero.')).toBeVisible();
  expect(screen.queryByRole('link', { name: 'Preview Invoice' })).not.toBeInTheDocument(); expect(alert).not.toHaveBeenCalled();
});
it('retains a valid owner-entered invoice and clears earlier inline errors', () => {
  render(<InvoiceGeneratorPage />); submit(); fill('客户 Example', 'Reviewed design work', '1250.50'); submit();
  expect(screen.queryByRole('alert')).not.toBeInTheDocument(); expect(alert).not.toHaveBeenCalled();
  const url = new URL(screen.getByRole('link', { name: 'Preview Invoice' }).getAttribute('href')!);
  expect(url.origin).toBe(window.location.origin); expect(url.pathname).toBe('/invoice-generator/view');
  const encoded = url.searchParams.get('data')!; const binary = atob(encoded.replace(/-/g, '+').replace(/_/g, '/'));
  const decoded = JSON.parse(new TextDecoder().decode(Uint8Array.from(binary, character => character.charCodeAt(0))));
  expect(decoded).toMatchObject({ clientName: '客户 Example', projectDetails: 'Reviewed design work', amount: '1250.50', baseCurrency: 'USD', transactionCurrency: 'USD' });
});
it('retires a generated invoice when edited fields are rejected on resubmission', () => {
  render(<InvoiceGeneratorPage />); fill('Original client', 'Original details', '1250.50'); submit();
  expect(screen.getByRole('link', { name: 'Preview Invoice' })).toBeVisible();
  fill('Revised client', 'Revised details', '0'); submit();
  expect(screen.getByLabelText('Client Name')).toHaveValue('Revised client'); expect(screen.getByLabelText('Project Details')).toHaveValue('Revised details');
  expect(screen.getByText('Enter a valid amount greater than zero.')).toBeVisible();
  expect(screen.queryByText('Your Invoice is Ready!')).not.toBeInTheDocument();
  expect(screen.queryByRole('link', { name: 'Preview Invoice' })).not.toBeInTheDocument(); expect(screen.queryByRole('button', { name: 'Copy Link' })).not.toBeInTheDocument();
  fireEvent.change(screen.getByLabelText('Amount ($)'), { target: { value: '15.00' } });
  expect(screen.queryByRole('link', { name: 'Preview Invoice' })).not.toBeInTheDocument(); submit();
  expect(screen.getByRole('link', { name: 'Preview Invoice' })).toBeVisible();
});
it.each(['client', 'details', 'amount', 'base currency', 'transaction currency', 'split enabled', 'split contact', 'split percentage'])('editing %s retires the generated snapshot before another submission', field => {
  render(<InvoiceGeneratorPage />); fill('Original client', 'Original details', '1250.50');
  fireEvent.click(screen.getByRole('checkbox', { name: 'Split this payment' }));
  fireEvent.change(screen.getByPlaceholderText('e.g. Sarah (Artist)'), { target: { value: 'Original partner' } }); submit();
  expect(screen.getByRole('link', { name: 'Preview Invoice' })).toBeVisible();
  const edit = (input: HTMLElement, value: string) => fireEvent.change(input, { target: { value } });
  if (field === 'client') edit(screen.getByLabelText('Client Name'), 'New client');
  if (field === 'details') edit(screen.getByLabelText('Project Details'), 'New details');
  if (field === 'amount') edit(screen.getByLabelText('Amount ($)'), '25.00');
  if (field === 'base currency') edit(screen.getByPlaceholderText('e.g. USD'), 'EUR');
  if (field === 'transaction currency') edit(screen.getByPlaceholderText('e.g. EUR'), 'EUR');
  if (field === 'split enabled') fireEvent.click(screen.getByRole('checkbox', { name: 'Split this payment' }));
  if (field === 'split contact') edit(screen.getByPlaceholderText('e.g. Sarah (Artist)'), 'New partner');
  if (field === 'split percentage') edit(screen.getByRole('slider'), '60');
  expect(screen.queryByText('Your Invoice is Ready!')).not.toBeInTheDocument(); expect(screen.queryByRole('link', { name: 'Preview Invoice' })).not.toBeInTheDocument(); expect(screen.queryByRole('button', { name: 'Copy Link' })).not.toBeInTheDocument();
});
