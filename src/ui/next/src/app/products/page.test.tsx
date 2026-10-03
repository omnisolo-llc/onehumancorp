import { render, screen, waitFor, within, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import ProductsPage from './page';

vi.mock('../components/AppShell', () => ({ AppShell: ({ children, subtitle, statusItems }: { children: React.ReactNode; subtitle?: string; statusItems?: Array<{ label: string; value: string }> }) => <main><p>{subtitle}</p>{statusItems?.map(item => <p key={item.label}>{item.label}: {item.value}</p>)}{children}</main> }));
const product = { id: 'real-product-id', title: 'Owner cake', description: 'Fresh vegan cake', item_type: 'Product', price_cents: 5000 };
afterEach(() => vi.unstubAllGlobals());

it('persists a manual product before showing success and reloads returned catalog rows', async () => {
  let products: typeof product[] = [];
  const transport = vi.fn(async (_url: string, options?: RequestInit) => {
    if (options?.method === 'POST') {
      products = [product];
      return Response.json({ success: true, product_id: product.id });
    }
    return Response.json(products);
  });
  vi.stubGlobal('fetch', transport);
  render(<ProductsPage />);
  const user = userEvent.setup();
  await user.click(screen.getByRole('button', { name: 'New Product' }));
  await user.type(screen.getByLabelText('Product Name'), product.title);
  await user.type(screen.getByLabelText('Description'), product.description);
  await user.clear(screen.getByLabelText('Price'));
  await user.type(screen.getByLabelText('Price'), '50.00');
  await user.click(screen.getByRole('button', { name: 'Save' }));
  expect(await screen.findByText('Product created')).toBeVisible();
  expect(screen.getByText(product.title)).toBeVisible();
  const mutation = transport.mock.calls.find(([, options]) => options?.method === 'POST');
  expect(mutation?.[0]).toBe('/api/v1/catalog/product');
  expect(JSON.parse(String(mutation?.[1]?.body))).toEqual({ name: product.title, description: product.description, price: '50.00', item_type: 'Product' });
  expect(screen.queryByText('Chocolate Cake')).toBeNull();
});

it('edits the real product ID and only shows the persisted updated price', async () => {
  let price = 5000;
  const transport = vi.fn(async (_url: string, options?: RequestInit) => {
    if (options?.method === 'PUT') {
      price = 4500;
      return Response.json({ success: true, product_id: product.id });
    }
    return Response.json([{ ...product, price_cents: price }]);
  });
  vi.stubGlobal('fetch', transport);
  render(<ProductsPage />);
  const user = userEvent.setup();
  await user.click(await screen.findByRole('button', { name: `Edit ${product.title}` }));
  expect(screen.getByLabelText('Product Name')).toHaveValue(product.title);
  expect(screen.getByLabelText('Description')).toHaveValue(product.description);
  await user.clear(screen.getByLabelText('Price'));
  await user.type(screen.getByLabelText('Price'), '45.00');
  await user.click(screen.getByRole('button', { name: 'Save' }));
  expect(await screen.findByText('Product updated')).toBeVisible();
  expect(screen.getByText('$45.00')).toBeVisible();
  const mutation = transport.mock.calls.find(([, options]) => options?.method === 'PUT');
  expect(mutation?.[0]).toBe(`/api/v1/catalog/product/${product.id}`);
  expect(JSON.parse(String(mutation?.[1]?.body))).toEqual({ name: product.title, description: product.description, price: '45.00' });
});

it.each([403, 404, 500])('keeps the editor and original price when saving returns %s', async (status) => {
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => options?.method === 'PUT'
    ? Response.json({ message: 'Product could not be saved' }, { status })
    : Response.json([product])));
  render(<ProductsPage />);
  const user = userEvent.setup();
  await user.click(await screen.findByRole('button', { name: `Edit ${product.title}` }));
  await user.clear(screen.getByLabelText('Price'));
  await user.type(screen.getByLabelText('Price'), '45.00');
  await user.click(screen.getByRole('button', { name: 'Save' }));
  expect(await within(screen.getByRole('dialog')).findByRole('alert')).toHaveTextContent('Product could not be saved');
  expect(screen.getByLabelText('Price')).toHaveValue('45.00');
  expect(screen.getByRole('dialog')).toBeVisible();
  expect(screen.getByText('$50.00')).toBeVisible();
  expect(screen.queryByText('Product updated')).toBeNull();
});

it('does not invent products when the catalog is unavailable', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({}, { status: 503 })));
  render(<ProductsPage />);
  expect(await screen.findByRole('alert')).toHaveTextContent('Failed to load products');
  expect(screen.queryByText('Chocolate Cake')).toBeNull();
});

it('disables duplicate submissions and preserves an unsaved edit on failure', async () => {
  let failSave: ((response: Response) => void) | undefined;
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => options?.method === 'PUT'
    ? new Promise<Response>((resolve) => { failSave = resolve; })
    : Response.json([product])));
  render(<ProductsPage />);
  const user = userEvent.setup();
  await user.click(await screen.findByRole('button', { name: `Edit ${product.title}` }));
  await user.click(screen.getByRole('button', { name: 'Save' }));
  await waitFor(() => expect(within(screen.getByRole('dialog')).getByRole('button', { name: 'Saving...' })).toBeDisabled());
  failSave?.(Response.json({ message: 'Try again' }, { status: 503 }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Try again');
  await user.click(screen.getByRole('button', { name: 'Cancel' }));
  expect(screen.queryByRole('dialog')).toBeNull();
});

describe('ProductsPage', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: async () => [{
        id: 'product-1',
        title: 'Seeded Tea',
        price_cents: 1250,
        image_url: '/seeded-tea.png',
      }],
    }));
  });

  it('renders correctly', () => {
    render(<ProductsPage />);
    expect(screen.getByText('Catalog Products')).toBeDefined();

  });

  it('does not invent product status and generates checkout QR data from the real product id', async () => {
    render(<ProductsPage />);

    expect(await screen.findByText('Seeded Tea')).toBeInTheDocument();
    expect(screen.queryByText('Active')).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Generate QR Code' }));
    const qr = screen.getByAltText('QR Code for Seeded Tea') as HTMLImageElement;
    const qrRequest = new URL(qr.src);
    expect(qrRequest.hostname).toBe('api.qrserver.com');
    expect(qrRequest.searchParams.get('data')).toBe('https://cloud.omnisolo.co/checkout?product_id=product-1');
  });

});

it('rejects a success response without a persisted product ID', async () => {
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options?: RequestInit) => options?.method === 'POST'
    ? Response.json({ success: true })
    : Response.json([])));
  render(<ProductsPage />);
  const user = userEvent.setup();
  await user.click(screen.getByRole('button', { name: 'New Product' }));
  await user.type(screen.getByLabelText('Product Name'), 'Unsaved cake');
  await user.click(screen.getByRole('button', { name: 'Save' }));
  expect(await within(screen.getByRole('dialog')).findByRole('alert')).toHaveTextContent('Product could not be saved');
  expect(screen.queryByText('Product created')).toBeNull();
});

it('does not invite a duplicate create when save succeeds but list refresh fails', async () => {
  let committed = false;
  const transport = vi.fn(async (_url: string, options?: RequestInit) => {
    if (options?.method === 'POST') {
      committed = true;
      return Response.json({ success: true, product_id: product.id });
    }
    return committed ? Response.json({}, { status: 503 }) : Response.json([]);
  });
  vi.stubGlobal('fetch', transport);
  render(<ProductsPage />);
  const user = userEvent.setup();
  await user.click(screen.getByRole('button', { name: 'New Product' }));
  await user.type(screen.getByLabelText('Product Name'), product.title);
  await user.click(screen.getByRole('button', { name: 'Save' }));
  expect(await screen.findByText('Product created')).toBeVisible();
  expect(await screen.findByRole('alert')).toHaveTextContent('Failed to load products');
  expect(screen.queryByRole('dialog')).toBeNull();
  expect(transport.mock.calls.filter(([, options]) => options?.method === 'POST')).toHaveLength(1);
});


it('describes persisted catalog rows without claiming a migration imported them', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json([product])));
  render(<ProductsPage />);
  expect(await screen.findByText(product.title)).toBeVisible();
  expect(screen.queryByText('Imported Products')).toBeNull();
  expect(screen.getByText('Catalog Products')).toBeVisible();
  expect(screen.getByText('Source: Catalog')).toBeVisible();
  expect(screen.getByText('Manage your saved catalog products.')).toBeVisible();
  expect(screen.queryByText(/staged from the migration/)).toBeNull();
});
