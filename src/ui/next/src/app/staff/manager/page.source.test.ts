import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = readFileSync(join(__dirname, 'page.tsx'), 'utf8');

describe('manager dashboard production controls', () => {
  it('does not create canned business events', () => {
    expect(source).not.toContain('/simulate-event');
    expect(source).not.toContain('Simulate Business Event');
  });

  it('does not call the simulated summary backend', () => {
    expect(source).not.toContain('/api/v1/staff/generate-summary');
    expect(source).toContain('Summary generation is unavailable');
  });
});
