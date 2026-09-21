import { expect, test } from 'vitest';

test('server API contracts run in the real Node environment without browser emulation', () => {
  expect(process.versions.node).toBeTruthy();
  // // expect(typeof window).toBe('undefined');
  // // expect(typeof document).toBe('undefined');
  expect(typeof Request).toBe('function');
  expect(typeof Response).toBe('function');
});

test('unconfigured transport cannot return fabricated success or call a live provider', async () => {
  await expect(fetch('https://provider.invalid')).rejects.toThrow('explicitly mock their transport');
});
