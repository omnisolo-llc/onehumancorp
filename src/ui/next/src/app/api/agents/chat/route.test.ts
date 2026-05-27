import { describe, it, expect } from 'vitest';
import { POST } from './route';

describe('agent chat route', () => {
  it('should be tested via POST', () => {
    expect(POST).toBeDefined();
  });
});