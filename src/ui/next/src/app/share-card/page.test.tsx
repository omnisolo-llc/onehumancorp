import { describe, expect, it, vi, afterEach, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import { renderToStaticMarkup } from 'react-dom/server';
import ShareCardPage, { generateMetadata } from './page';

const originalLocation = window.location;
afterEach(() => { Object.defineProperty(window, 'location', { configurable: true, writable: true, value: originalLocation }); });
beforeEach(() => Object.defineProperty(window, 'location', { configurable: true, writable: true, value: { ...originalLocation, replace: vi.fn() } }));

const target = async (url: string) => (await generateMetadata({ searchParams: Promise.resolve({ url }) })).openGraph;
describe('share-card destinations', () => {
  it.each([
    ['relative app route', '/help/getting-started-1?view=plain#intro', '/help/getting-started-1?view=plain#intro'],
    ['relative text route', 'onboarding?ref=owner', '/onboarding?ref=owner'],
    ['trusted cloud HTTPS', 'https://cloud.omnisolo.co/onboarding?ref=owner', 'https://cloud.omnisolo.co/onboarding?ref=owner'],
    ['canonical HTTPS port', 'https://cloud.omnisolo.co:443/onboarding', 'https://cloud.omnisolo.co/onboarding'],
    ['existing local app URL', 'http://localhost:3000/dashboard?view=compact', '/dashboard?view=compact'],
    ['trusted root HTTPS', 'https://omnisolo.co/onboarding', 'https://omnisolo.co/onboarding'],
  ])('retains %s', async (_label, input, expected) => {
    expect(await target(input)).toMatchObject({ url: expected });
  });
  it.each([
    'ftp://cloud.omnisolo.co/onboarding',
    'http://cloud.omnisolo.co/onboarding',
    'https://fixture-user:fixture-password@cloud.omnisolo.co/onboarding',
    'https://cloud.omnisolo.co:9443/onboarding',
    '/share-card', '/share-card?url=%2Fshare-card', '/%73hare-card/', '/%2Fshare-card',
    'https://cloud.omnisolo.co/share-card?url=%2Fshare-card',
    'javascript:alert(1)', '//untrusted.invalid/path',
    'http://localhost:3000//untrusted.invalid/path',
    'http://localhost:3000///untrusted.invalid/path',
    'https://cloud.omnisolo.co.untrusted.invalid/path',
    '/%5cshare-card?url=%2Fonboarding', '/help%00page', '/help%0apage',
  ])('rejects unsupported or looping target %s', async input => {
    expect(await target(input)).toMatchObject({ url: '/onboarding' });
  });
});

it('preserves real query separators in the visible fallback destination', async () => {
  render(await ShareCardPage({ searchParams: Promise.resolve({ url: '/onboarding?ref=owner&mode=manual' }) }));
  expect(screen.getByRole('link')).toHaveAttribute('href', '/onboarding?ref=owner&mode=manual');
});

it('does not schedule a native refresh before the redirect component is hydrated', async () => {
  const html = renderToStaticMarkup(await ShareCardPage({ searchParams: Promise.resolve({}) }));
  expect(html).not.toMatch(/http-equiv=["']refresh["']/i);
  expect(html).toContain('href="/onboarding"');
});

it('the actual page keeps its visible link on the normalized target without a native refresh', async () => {
  const output = await ShareCardPage({ searchParams: Promise.resolve({ url: 'https://fixture-user:fixture-password@cloud.omnisolo.co/path' }) });
  const { container } = render(output);
  expect(screen.getByRole('link')).toHaveAttribute('href', '/onboarding');
  expect(document.querySelector('meta[http-equiv="refresh"]')).toBeNull();
  expect(container.querySelector('script')).toBeNull();
});

it('a local URL cannot become an external network-path URL in rendered redirect surfaces', async () => {
  const output = await ShareCardPage({ searchParams: Promise.resolve({ url: 'http://localhost:3000//untrusted.invalid/path' }) });
  const { container } = render(output);
  expect(screen.getByRole('link')).toHaveAttribute('href', '/onboarding');
  expect(document.querySelector('meta[http-equiv="refresh"]')).toBeNull();
  expect(container.querySelector('script')).toBeNull();
});

it('schedules only one automatic document navigation so the destination cannot be replaced twice', async () => {
  const { container } = render(await ShareCardPage({ searchParams: Promise.resolve({}) }));
  const refreshes = document.querySelectorAll('meta[http-equiv="refresh"]');
  const scriptNavigations = Array.from(container.querySelectorAll('script')).filter(script => /location\.(?:replace|assign)|location\s*=/.test(script.textContent || ''));
  expect(refreshes.length + scriptNavigations.length).toBe(0);
  expect(screen.getByRole('link')).toHaveAttribute('href', '/onboarding');
});
