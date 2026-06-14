import { expect, test } from 'vitest';
import { isPowerSyncSupportedForLocation } from './PowerSyncProvider';

test('allows PowerSync on secure browser contexts', () => {
  expect(isPowerSyncSupportedForLocation(true, '172.17.197.51')).toBe(true);
});

test('allows PowerSync on localhost even when served over http', () => {
  expect(isPowerSyncSupportedForLocation(false, 'localhost')).toBe(true);
  expect(isPowerSyncSupportedForLocation(false, '127.0.0.1')).toBe(true);
});

test('disables PowerSync on insecure IP-hosted pages', () => {
  expect(isPowerSyncSupportedForLocation(false, '172.17.197.51')).toBe(false);
});
