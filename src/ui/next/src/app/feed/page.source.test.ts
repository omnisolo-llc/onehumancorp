import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const feedPageSource = readFileSync(join(__dirname, 'page.tsx'), 'utf8');

describe('feed simulation controls', () => {
  it('does not ship controls or endpoints that create fabricated feed records', () => {
    expect(feedPageSource).not.toMatch(/simulate[A-Z]/);
    expect(feedPageSource).not.toContain('/simulate-');
    expect(feedPageSource).not.toContain('Simulate ');
  });
});
