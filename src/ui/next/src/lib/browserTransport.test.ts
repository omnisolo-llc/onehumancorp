import { expect, test } from 'vitest';

// This runs in the browser/jsdom project, not the Node API-test project.
test('browser unit tests expose a real DOM but do not invent API success', async () => {
  expect(typeof document.createElement).toBe('function');
  await expect(fetch('/api/v1/billing/usage')).rejects.toThrow('explicitly mock their transport boundary');
});

test('unspecified absolute requests never fall through to native network transport', async () => {
  for (const target of ['https://provider.invalid/v1', new URL('https://provider.invalid/v1'),
    new Request('https://provider.invalid/v1')]) {
    await expect(fetch(target)).rejects.toThrow('explicitly mock their transport boundary');
  }
});
