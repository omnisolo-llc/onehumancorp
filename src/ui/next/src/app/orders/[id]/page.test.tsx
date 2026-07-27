import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { act } from 'react';
import OrderDetailsPage from './page';

vi.mock('next/navigation', () => ({ useParams: () => ({ id: 'order-1' }) }));
vi.mock('../../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <main className="app-main">{children}</main>,
}));

describe('OrderDetailsPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.startsWith('/api/v1/ui/orders')) {
        return Promise.resolve({ ok: true, json: async () => [{ id: 'order-1', customer_name: 'A Customer', total_amount: 45, status: 'paid', created_at: '2026-07-17' }] });
      }
      if (url === '/api/v1/shipping/rates') {
        return Promise.resolve({ ok: true, json: async () => ({ rates: [{ id: 'rate-1', carrier: 'UPS', service: 'Ground', amount: '12.50', days: 3 }] }) });
      }
      if (url === '/api/v1/shipping/label') {
        return Promise.resolve({ ok: true, json: async () => ({ success: true, labelUrl: 'https://shippo-delivery-east.s3.amazonaws.com/order-1.pdf', trackingNumber: '1Z999', carrier: 'UPS' }) });
      }
      return Promise.resolve({ ok: false, json: async () => ({}) });
    });
  });

  it('restores validated shipping rate and label purchase flow', async () => {
    act(() => { render(<OrderDetailsPage />); });
    expect(await screen.findByText('A Customer')).toBeDefined();

    expect((screen.getByLabelText('Package weight in ounces') as HTMLInputElement).value).toBe('');
    expect(screen.getByLabelText('Package dimensions')).toHaveValue('');

    fireEvent.change(screen.getByLabelText('Package weight in ounces'), { target: { value: '16' } });
    fireEvent.change(screen.getByLabelText('Package dimensions'), { target: { value: '10x8x6' } });
    fireEvent.click(screen.getByRole('button', { name: 'Get Shipping Rates' }));
    expect(await screen.findByText('UPS Ground')).toBeDefined();
    expect(screen.getByText('$12.50')).toBeDefined();

    const rateCall = vi.mocked(global.fetch).mock.calls.find(([url]) => url === '/api/v1/shipping/rates');
    expect(JSON.parse(String(rateCall?.[1]?.body))).toEqual({ orderId: 'order-1', weight: '16', dimensions: '10x8x6' });

    fireEvent.click(screen.getByRole('radio', { name: /UPS Ground/ }));
    fireEvent.click(screen.getByRole('button', { name: 'Buy Label' }));
    expect(await screen.findByText('1Z999')).toBeDefined();
    expect(screen.getByRole('link', { name: 'Open Shipping Label' }).getAttribute('href')).toBe('https://shippo-delivery-east.s3.amazonaws.com/order-1.pdf');
  });

  it('accepts the Rust string amount contract', async () => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.startsWith('/api/v1/ui/orders')) {
        return Promise.resolve({ ok: true, json: async () => [{ id: 'order-1' }] });
      }
      return Promise.resolve({ ok: true, json: async () => ({ rates: [{ id: 'rate-1', carrier: 'UPS', service: 'Ground', amount: '12.50', days: 3 }] }) });
    });
    act(() => { render(<OrderDetailsPage />); });
    await screen.findByText('order-1');
    fireEvent.change(screen.getByLabelText('Package weight in ounces'), { target: { value: '16' } });
    fireEvent.change(screen.getByLabelText('Package dimensions'), { target: { value: '10x8x6' } });
    fireEvent.click(screen.getByRole('button', { name: 'Get Shipping Rates' }));

    expect(await screen.findByText('$12.50')).toBeDefined();
  });

  it('rejects a label URL outside the trusted Shippo delivery hosts', async () => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.startsWith('/api/v1/ui/orders')) return Promise.resolve({ ok: true, json: async () => [{ id: 'order-1' }] });
      if (url === '/api/v1/shipping/rates') return Promise.resolve({ ok: true, json: async () => ({ rates: [{ id: 'rate-1', carrier: 'UPS', service: 'Ground', amount: '12.50', days: 3 }] }) });
      return Promise.resolve({ ok: true, json: async () => ({ success: true, labelUrl: 'https://attacker.example/order.pdf', trackingNumber: '1Z999', carrier: 'UPS' }) });
    });
    act(() => { render(<OrderDetailsPage />); });
    await screen.findByText('order-1');
    fireEvent.change(screen.getByLabelText('Package weight in ounces'), { target: { value: '16' } });
    fireEvent.change(screen.getByLabelText('Package dimensions'), { target: { value: '10x8x6' } });
    fireEvent.click(screen.getByRole('button', { name: 'Get Shipping Rates' }));
    fireEvent.click(await screen.findByRole('radio', { name: /UPS Ground/ }));
    fireEvent.click(screen.getByRole('button', { name: 'Buy Label' }));

    expect(await screen.findByRole('alert')).toHaveTextContent('The shipping label could not be confirmed.');
    expect(screen.queryByRole('link', { name: 'Open Shipping Label' })).toBeNull();
  });

  it('rejects malformed order fields and malformed rates', async () => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.startsWith('/api/v1/ui/orders')) {
        return Promise.resolve({ ok: true, json: async () => [{ id: 'order-1', customer_name: { fake: true }, total_amount: '45', status: ['paid'], created_at: 123 }] });
      }
      return Promise.resolve({ ok: true, json: async () => ({ rates: [{ id: '', carrier: {}, amount: -1 }] }) });
    });
    act(() => { render(<OrderDetailsPage />); });
    expect(await screen.findAllByText('Unavailable')).toHaveLength(4);
    fireEvent.change(screen.getByLabelText('Package weight in ounces'), { target: { value: '16' } });
    fireEvent.change(screen.getByLabelText('Package dimensions'), { target: { value: '10x8x6' } });
    fireEvent.click(screen.getByRole('button', { name: 'Get Shipping Rates' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('Shipping rates are unavailable.');
  });

  it('keeps the product shell visible when the database has no matching order', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => [] });

    let container: any;
    act(() => { container = render(<OrderDetailsPage />).container; });

    expect(await screen.findByText('This order was not found.')).toBeInTheDocument();
    expect(container.querySelector('.app-main')).not.toBeNull();
  });
});
