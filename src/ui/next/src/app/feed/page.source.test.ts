import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const feedPageSource = readFileSync(join(__dirname, 'page.tsx'), 'utf8');

describe('feed simulation controls', () => {
  it('owns the invoice draft simulator exactly once', () => {
    expect(feedPageSource.match(/const simulateInvoiceDraft =/g)).toHaveLength(1);
    expect(feedPageSource.match(/data-testid="simulate-invoice-draft-btn"/g)).toHaveLength(1);
  });
});
