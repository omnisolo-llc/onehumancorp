import { render, screen } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import PublicBioPage from './page';

vi.mock('next/navigation', () => ({ useParams: () => ({ tenant: 'test-store' }) }));

beforeEach(() => { vi.clearAllMocks(); });

test.each(['javascript:alert(1)', 'data:text/html,test', '//elsewhere.test', 'https://', '',
  'https://user:secret@example.test', '/\\elsewhere.test', 'java\nscript:alert(1)'])('keeps an invalid saved bio destination %j non-interactive', async (url) => {
  global.fetch = vi.fn().mockResolvedValue(Response.json({
    store_name: 'Test Store', bio: 'Our public profile', theme: 'light',
    links: [{ id: 'unsafe', title: 'Unsafe destination', url },
      { id: 'safe', title: 'Book a consultation', url: '/booking' }],
  }));
  render(<PublicBioPage />);

  expect(await screen.findByText('Unsafe destination')).toBeVisible();
  expect(screen.queryByRole('link', { name: 'Unsafe destination' })).not.toBeInTheDocument();
  expect(screen.getByRole('link', { name: 'Book a consultation' })).toHaveAttribute('href', '/booking');
});

test('preserves the intended HTTP(S) destination and safe new-tab behavior', async () => {
  global.fetch = vi.fn().mockResolvedValue(Response.json({
    store_name: 'Test Store', bio: 'Our public profile', theme: 'light',
    links: [{ id: 'external', title: 'Visit our store', url: 'https://store.example.test/shop' }],
  }));
  render(<PublicBioPage />);
  const link = await screen.findByRole('link', { name: 'Visit our store' });
  expect(link).toHaveAttribute('href', 'https://store.example.test/shop');
  expect(link).toHaveAttribute('target', '_blank');
  expect(link).toHaveAttribute('rel', 'noopener noreferrer');
});
