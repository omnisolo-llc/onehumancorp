import { describe, expect, it } from 'vitest';
import { isRecord, recordOrEmpty } from './records';

describe('untrusted JSON object boundaries', () => {
  it.each([null, undefined, false, 2, 'text', [], [1]])('rejects non-record %j', value => {
    expect(isRecord(value)).toBe(false);
    expect(recordOrEmpty(value)).toEqual({});
  });
  it('keeps named record fields without coercing primitives into business data', () => {
    const record = { amount: 42, description: 'Actual value', nested: { id: 'x' } };
    expect(isRecord(record)).toBe(true);
    expect(recordOrEmpty(record)).toBe(record);
    expect(recordOrEmpty(Object.create(null))).toEqual({});
  });
});
