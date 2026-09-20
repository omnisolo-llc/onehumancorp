import { describe, expect, it } from 'vitest';
import { errorMessage } from './errors';

describe('errorMessage', () => {
  it('retains Error messages and explicit nonempty string failures', () => {
    expect(errorMessage(new Error('Connection unavailable'))).toBe('Connection unavailable');
    expect(errorMessage('Connection unavailable')).toBe('Connection unavailable');
  });

  it('narrows unknown objects instead of treating every thrown value as an Error', () => {
    expect(errorMessage({ message: 'Provider unavailable' })).toBe('Provider unavailable');
    for (const value of [null, undefined, 12, false, {}, { message: 12 }, { message: '' }, '  ']) {
      expect(errorMessage(value, 'Could not save the draft')).toBe('Could not save the draft');
    }
  });

  it('does not throw again when an error object has a throwing message getter', () => {
    const value = Object.defineProperty({}, 'message', { get() { throw new Error('unreadable'); } });
    expect(errorMessage(value, 'Request failed')).toBe('Request failed');
  });

  it('does not stringify arbitrary objects or include their other properties', () => {
    const value = { token: 'must-not-be-shown', toString() { throw new Error('not a message'); } };
    expect(errorMessage(value)).toBe('The request could not be completed.');
  });
});
