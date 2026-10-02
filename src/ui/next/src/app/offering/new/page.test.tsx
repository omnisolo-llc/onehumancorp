import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, expect, it, vi } from 'vitest';
import NewOfferingPage from './page';

const offering = { title: 'Beginner Guitar Lesson', description: 'A one-hour lesson', item_type: 'Service', price: '50.00' };
afterEach(() => vi.unstubAllGlobals());

it('publishes the provider-generated offering with the price reviewed by its owner', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => Response.json(url.endsWith('/generate') ? offering : { success: true })));
  render(<NewOfferingPage />);
  const user = userEvent.setup();
  await user.type(screen.getByPlaceholderText('e.g. Guitar lessons for beginners, 1 hour'), 'Guitar lessons for beginners, 1 hour');
  await user.click(screen.getByRole('button', { name: 'Generate Details' }));
  const price = await screen.findByDisplayValue('50.00');
  await user.clear(price);
  await user.type(price, '45.00');
  await user.click(screen.getByRole('button', { name: 'Publish Offering' }));
  expect(await screen.findByRole('heading', { name: 'Offering Published!' })).toBeVisible();
  const [url, options] = vi.mocked(fetch).mock.calls[1];
  expect(url).toBe('/api/v1/catalog/product');
  expect(JSON.parse(String(options?.body))).toEqual({ name: offering.title, description: offering.description, item_type: 'Service', price: '45.00' });
});

it('keeps the owner intent without inventing an offering when the provider is unavailable', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ error: 'offering generation unavailable' }, { status: 503 })));
  render(<NewOfferingPage />);
  const user = userEvent.setup();
  const intent = screen.getByPlaceholderText('e.g. Guitar lessons for beginners, 1 hour');
  await user.type(intent, 'Owner-written service');
  await user.click(screen.getByRole('button', { name: 'Generate Details' }));
  expect(await screen.findByRole('status')).toHaveTextContent('Offering generation is unavailable.');
  expect(intent).toHaveValue('Owner-written service');
  expect(screen.queryByRole('button', { name: 'Publish Offering' })).toBeNull();
});

it('retains the reviewed offering and does not claim publication after a rejected save', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url.endsWith('/generate')
    ? Response.json(offering)
    : Response.json({ error: 'unavailable' }, { status: 500 })));
  render(<NewOfferingPage />);
  const user = userEvent.setup();
  await user.type(screen.getByPlaceholderText('e.g. Guitar lessons for beginners, 1 hour'), 'Owner-written service');
  await user.click(screen.getByRole('button', { name: 'Generate Details' }));
  await user.click(await screen.findByRole('button', { name: 'Publish Offering' }));
  expect(await screen.findByRole('status')).toHaveTextContent('Offering publishing is unavailable.');
  expect(screen.getByDisplayValue(offering.title)).toBeVisible();
  expect(screen.queryByRole('heading', { name: 'Offering Published!' })).toBeNull();
});
