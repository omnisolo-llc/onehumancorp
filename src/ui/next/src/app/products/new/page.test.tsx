import { act, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, expect, it, vi } from 'vitest';
import AutoCatalogPage from './page';

const offering = { title: 'Beginner Guitar Lesson', description: 'A one-hour lesson', item_type: 'Service', price: '50.00' };
afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

it('creates a provider-generated offering at the owner-reviewed price through the current product flow', async () => {
  let finishPublishing!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url.endsWith('/generate')
    ? Response.json(offering)
    : new Promise<Response>((resolve) => { finishPublishing = resolve; })));
  render(<AutoCatalogPage />);
  const user = userEvent.setup();
  await user.click(screen.getByRole('button', { name: 'Or describe your offering' }));
  await user.type(screen.getByPlaceholderText('e.g., Guitar lessons for beginners, 1 hour'), 'Guitar lessons for beginners, 1 hour');
  await user.click(screen.getByRole('button', { name: 'Generate' }));
  const price = await screen.findByDisplayValue('50.00');
  await user.clear(price);
  await user.type(price, '45.00');
  await user.click(screen.getByRole('button', { name: 'Looks Good' }));
  // Keep user-event on the real clock; control only the publishing response
  // and animation timers, after the owner has finished reviewing the offering.
  vi.useFakeTimers();
  await act(async () => { await vi.advanceTimersByTimeAsync(3500); });
  expect(screen.getByText('Updating storefront...')).toBeVisible();
  expect(screen.queryByRole('heading', { name: 'Product Published!' })).toBeNull();
  await act(async () => { finishPublishing(Response.json({ success: true })); });
  // Drive both publishing stages without spending 3.5 seconds of the test budget
  // on presentation timers. Check each boundary so completion cannot be skipped.
  await act(async () => { await vi.advanceTimersByTimeAsync(1499); });
  expect(screen.getByText('Updating storefront...')).toBeVisible();
  expect(screen.queryByRole('heading', { name: 'Product Published!' })).toBeNull();
  await act(async () => { await vi.advanceTimersByTimeAsync(1); });
  expect(screen.getByText('Storefront updated and optimized for search.')).toBeVisible();
  await act(async () => { await vi.advanceTimersByTimeAsync(1999); });
  expect(screen.queryByRole('heading', { name: 'Product Published!' })).toBeNull();
  await act(async () => { await vi.advanceTimersByTimeAsync(1); });
  expect(screen.getByRole('heading', { name: 'Product Published!' })).toBeVisible();
  const [url, options] = vi.mocked(fetch).mock.calls[1];
  expect(url).toBe('/api/v1/catalog/product');
  expect(JSON.parse(String(options?.body))).toMatchObject({ name: offering.title, description: offering.description, item_type: 'Service', price: '45.00' });
});

it('shows the real provider failure without inventing a product or publish control', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ message: 'The AI provider is not configured.' }, { status: 503 })));
  render(<AutoCatalogPage />);
  const user = userEvent.setup();
  await user.click(screen.getByRole('button', { name: 'Or describe your offering' }));
  await user.type(screen.getByPlaceholderText('e.g., Guitar lessons for beginners, 1 hour'), 'Owner-written service');
  await user.click(screen.getByRole('button', { name: 'Generate' }));
  expect(await screen.findByText('The AI provider is not configured.')).toBeVisible();
  expect(screen.getByPlaceholderText('e.g., Guitar lessons for beginners, 1 hour')).toHaveValue('Owner-written service');
  expect(screen.queryByRole('button', { name: 'Looks Good' })).toBeNull();
});
