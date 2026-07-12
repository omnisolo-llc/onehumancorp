// @vitest-environment node

import React from 'react';
import { renderToString } from 'react-dom/server';
import { expect, test, vi } from 'vitest';
import { PowerSyncProvider } from './PowerSyncProvider';

vi.mock('./db', () => ({
  getPowerSyncDB: vi.fn(),
}));

test('renders the stable loading fallback on the server', () => {
  const html = renderToString(
    <PowerSyncProvider fallback={<div>Stable loading state</div>} unsupportedFallback={<div>API fallback</div>}>
      <div>Inbox content</div>
    </PowerSyncProvider>,
  );

  expect(html).toContain('Stable loading state');
  expect(html).not.toContain('API fallback');
});
