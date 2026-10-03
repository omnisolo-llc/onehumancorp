import { StrictMode } from 'react';
import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, expect, test, vi } from 'vitest';
import RedirectAfterHydration, { redirectSharedDocument } from './RedirectAfterHydration';

const location = window.location;
afterEach(() => { cleanup(); Object.defineProperty(window, 'location', { configurable: true, writable: true, value: location }); });

test('hydration replay and remount schedule one navigation per document and retain the actual fallback link', () => {
  const replace = vi.fn();
  Object.defineProperty(window, 'location', { configurable: true, writable: true, value: { ...location, replace } });
  const view = render(<StrictMode><RedirectAfterHydration targetUrl="/onboarding?ref=owner&mode=manual" /></StrictMode>);
  expect(replace).toHaveBeenCalledExactlyOnceWith('/onboarding?ref=owner&mode=manual');
  expect(screen.getByRole('link')).toHaveAttribute('href', '/onboarding?ref=owner&mode=manual');
  view.unmount();
  render(<RedirectAfterHydration targetUrl="/onboarding?ref=owner&mode=manual" />);
  expect(replace).toHaveBeenCalledTimes(1);
  expect(document.querySelector('meta[http-equiv="refresh"]')).toBeNull();
});

test('separate documents have independent redirect attempts', () => {
  const replace = vi.fn();
  const first = document.implementation.createHTMLDocument();
  const second = document.implementation.createHTMLDocument();
  redirectSharedDocument(first, '/onboarding', replace);
  redirectSharedDocument(first, '/onboarding', replace);
  redirectSharedDocument(second, '/help', replace);
  expect(replace.mock.calls).toEqual([['/onboarding'], ['/help']]);
});

test('an uncertain navigation error is never automatically retried', () => {
  const replace = vi.fn(() => { throw new Error('navigation denied'); });
  const current = document.implementation.createHTMLDocument();
  expect(() => redirectSharedDocument(current, '/onboarding', replace)).toThrow('navigation denied');
  redirectSharedDocument(current, '/onboarding', replace);
  expect(replace).toHaveBeenCalledTimes(1);
});
