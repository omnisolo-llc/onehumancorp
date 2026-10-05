import { expect, it } from 'vitest';
import type { Viewport } from 'next';
import { viewport } from './layout';

it('keeps device-width rendering while allowing the browser to magnify the page', () => {
  const configuration: Viewport = viewport;
  expect(configuration.width).toBe('device-width');
  expect(configuration.initialScale).toBe(1);
  expect(configuration.maximumScale).toBeUndefined();
  expect(configuration.userScalable).not.toBe(false);
});
