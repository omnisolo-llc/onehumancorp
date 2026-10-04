import { act,fireEvent,render,screen } from '@testing-library/react';
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
      if (url === '/api/v1/ui/orders/order-1') {
        return Promise.resolve({ ok: true, json: async () => ({ id: 'order-1', customer_name: 'A Customer', total_amount: 45, status: 'paid', created_at: '2026-07-17' }) });
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
      if (url === '/api/v1/ui/orders/order-1') {
        return Promise.resolve({ ok: true, json: async () => ({ id: 'order-1' }) });
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
      if (url === '/api/v1/ui/orders/order-1') return Promise.resolve({ ok: true, json: async () => ({ id: 'order-1' }) });
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
      if (url === '/api/v1/ui/orders/order-1') {
        return Promise.resolve({ ok: true, json: async () => ({ id: 'order-1', customer_name: { fake: true }, total_amount: '45', status: ['paid'], created_at: 123 }) });
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
    global.fetch = vi.fn().mockResolvedValue({ ok: false, status: 404, json: async () => ({ error: 'Order not found' }) });

    const { container } = render(<OrderDetailsPage />);

    expect(await screen.findByText('This order was not found.')).toBeInTheDocument();
    expect(container.querySelector('.app-main')).not.toBeNull();
  });
});

describe('Shipping purchase evidence',()=>{
  it('never fabricates a customer or shipping capability for a fixture-shaped order ID',async()=>{route.id='e2e-shippo-order';global.fetch=vi.fn().mockRejectedValue(new Error('offline'));render(<OrderDetailsPage/>);expect(await screen.findByRole('alert')).toHaveTextContent('Order data is unavailable');expect(screen.queryByText('Alice Johnson')).toBeNull();});
  const ready = async (receipt: Record<string,unknown>) => {
    global.fetch=vi.fn().mockImplementation(async(url:string)=>{
      if(url === '/api/v1/ui/orders/order-1')return new Response(JSON.stringify({id:'order-1',customer_name:'A Customer',status:'paid'}));
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


describe('Authoritative order detail', () => {
  it('reads the exact order even when it is absent from the newest-fifty list', async () => {
    global.fetch = vi.fn(async (url: string) => new Response(JSON.stringify(url === '/api/v1/ui/orders/order-1'
      ? { id: 'order-1', customer_name: 'Older recorded customer', status: 'pending' }
      : Array.from({ length: 50 }, (_, i) => ({ id: `newer-${i}` })))));
    render(<OrderDetailsPage />);
    expect(await screen.findByText('Older recorded customer')).toBeVisible();
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/ui/orders/order-1', expect.objectContaining({ cache: 'no-store', signal: expect.any(AbortSignal) }));
    expect(vi.mocked(global.fetch).mock.calls.some(([url]) => url === '/api/v1/ui/orders')).toBe(false);
  });

  it.each([[], { id: 'another-order', customer_name: 'Foreign receipt' }])('rejects a successful but mismatched detail body', async body => {
    global.fetch = vi.fn(async () => Response.json(body));
    render(<OrderDetailsPage />);
    expect(await screen.findByRole('alert')).toHaveTextContent('Order data is unavailable.');
    expect(screen.queryByText('Foreign receipt')).toBeNull();
    expect(screen.queryByRole('button', { name: 'Get Shipping Rates' })).toBeNull();
  });

  it('does not render an older in-flight order after navigation', async () => {
    let resolveFirst!: (response: Response) => void;
    global.fetch = vi.fn((url: string) => url === '/api/v1/ui/orders/order-1'
      ? new Promise<Response>(resolve => { resolveFirst = resolve; })
      : Promise.resolve(Response.json({ id: 'order-2', customer_name: 'Current customer' })));
    const view = render(<OrderDetailsPage />);
    route.id = 'order-2'; view.rerender(<OrderDetailsPage />);
    expect(await screen.findByText('Current customer')).toBeVisible();
    await act(async () => { resolveFirst(Response.json({ id: 'order-1', customer_name: 'Stale customer' })); });
    expect(screen.queryByText('Stale customer')).toBeNull();
    expect(screen.getByText('Current customer')).toBeVisible();
  });
});


it('never shows a previous order label when its purchase finishes after navigation', async () => {
  let resolvePurchase!: (response: Response) => void;
  global.fetch = vi.fn((url: string) => {
    if (url.startsWith('/api/v1/ui/orders/')) return Promise.resolve(Response.json({ id: url.split('/').at(-1), customer_name: route.id === 'order-1' ? 'First customer' : 'Current customer' }));
    if (url === '/api/v1/shipping/rates') return Promise.resolve(Response.json({ rates: [{ id: 'rate-1', carrier: 'UPS', service: 'Ground', amount: '12.50' }] }));
    return new Promise<Response>(resolve => { resolvePurchase = resolve; });
  });
  const view = render(<OrderDetailsPage />); await screen.findByText('First customer');
  fireEvent.change(screen.getByLabelText('Package weight in ounces'), { target: { value: '16' } });
  fireEvent.change(screen.getByLabelText('Package dimensions'), { target: { value: '10x8x6' } });
  fireEvent.click(screen.getByRole('button', { name: 'Get Shipping Rates' })); await screen.findByText('UPS Ground');
  fireEvent.click(screen.getByRole('button', { name: 'Buy Label' }));
  route.id = 'order-2'; view.rerender(<OrderDetailsPage />); await screen.findByText('Current customer');
  await act(async () => { resolvePurchase(Response.json({ success: true, labelUrl: 'https://app.goshippo.com/previous-order.pdf', trackingNumber: 'old-tracking' })); });
  expect(screen.queryByRole('link', { name: 'Open Shipping Label' })).toBeNull();
  expect(screen.queryByText('old-tracking')).toBeNull();
  expect(screen.getByRole('button', { name: 'Get Shipping Rates' })).toBeEnabled();
});
