import { fireEvent,render,screen } from '@testing-library/react';
import { afterEach,beforeEach,describe,expect,it,vi } from 'vitest';
import OrderDetailsPage from './page';

const route=vi.hoisted(()=>({id:'order-1'}));
vi.mock('next/navigation', () => ({ useParams: () => ({ id: route.id }) }));
afterEach(()=>{route.id='order-1';});
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
    render(<OrderDetailsPage />);
    expect(await screen.findByText('A Customer')).toBeDefined();

    expect(screen.getByLabelText('Package weight in ounces')).toHaveValue(null);
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
    render(<OrderDetailsPage />);
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
    render(<OrderDetailsPage />);
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
    render(<OrderDetailsPage />);
    expect(await screen.findAllByText('Unavailable')).toHaveLength(4);
    fireEvent.change(screen.getByLabelText('Package weight in ounces'), { target: { value: '16' } });
    fireEvent.change(screen.getByLabelText('Package dimensions'), { target: { value: '10x8x6' } });
    fireEvent.click(screen.getByRole('button', { name: 'Get Shipping Rates' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('Shipping rates are unavailable.');
  });

  it('keeps the product shell visible when the database has no matching order', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => [] });

    const { container } = render(<OrderDetailsPage />);

    expect(await screen.findByText('This order was not found.')).toBeInTheDocument();
    expect(container.querySelector('.app-main')).not.toBeNull();
  });
});

describe('Shipping purchase evidence',()=>{
  it('never fabricates a customer or shipping capability for a fixture-shaped order ID',async()=>{route.id='e2e-shippo-order';global.fetch=vi.fn().mockRejectedValue(new Error('offline'));render(<OrderDetailsPage/>);expect(await screen.findByRole('alert')).toHaveTextContent('Order data is unavailable');expect(screen.queryByText('Alice Johnson')).toBeNull();});
  const ready = async (receipt: Record<string,unknown>) => {
    global.fetch=vi.fn().mockImplementation(async(url:string)=>{
      if(url.startsWith('/api/v1/ui/orders'))return new Response(JSON.stringify([{id:'order-1',customer_name:'A Customer',status:'paid'}]));
      if(url==='/api/v1/shipping/rates')return new Response(JSON.stringify({rates:[{id:'rate-1',carrier:'UPS',service:'Ground',amount:'12.50'}]}));
      return new Response(JSON.stringify(receipt),{status:receipt.success===true?200:202});
    });
    render(<OrderDetailsPage/>);await screen.findByText('A Customer');
    fireEvent.change(screen.getByLabelText('Package weight in ounces'),{target:{value:'16'}});fireEvent.change(screen.getByLabelText('Package dimensions'),{target:{value:'10x8x6'}});fireEvent.click(screen.getByRole('button',{name:'Get Shipping Rates'}));await screen.findByText('UPS Ground');fireEvent.click(screen.getByRole('button',{name:'Buy Label'}));
  };
  it('does not turn a purchased label into a shipped order',async()=>{await ready({success:true,labelUrl:'https://app.goshippo.com/label.pdf',trackingNumber:'actual-tracking',carrier:'ups',transactionId:'txn_a',test:true});await screen.findByRole('link',{name:'Open Shipping Label'});expect(screen.queryByText('Shipped')).toBeNull();expect(screen.getByText('paid')).toBeVisible();});
  it('preserves a real label whose optional tracking and carrier are not yet available',async()=>{await ready({success:true,labelUrl:'https://app.goshippo.com/label.pdf',trackingNumber:null,carrier:null,transactionId:'txn_a',test:true});expect(await screen.findByRole('link',{name:'Open Shipping Label'})).toHaveAttribute('href','https://app.goshippo.com/label.pdf');expect(screen.getByText('Not available yet')).toBeVisible();});
  it('blocks a second purchase after a provider outcome requiring reconciliation',async()=>{await ready({success:false,status:'outcome_unknown',reconciliationRequired:true,error:'Reconcile before retrying',transactionId:'txn_a'});expect(await screen.findByRole('alert')).toHaveTextContent('Reconcile before retrying');expect(screen.getByRole('button',{name:'Buy Label'})).toBeDisabled();});
});
