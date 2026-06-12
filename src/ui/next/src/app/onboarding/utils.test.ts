import { generateSubdomain } from './utils';

describe('generateSubdomain', () => {
  it('returns default subdomain when name is empty', () => {
    expect(generateSubdomain('')).toBe('my-business.ohc.app');
    expect(generateSubdomain('   ')).toBe('my-business.ohc.app');
    expect(generateSubdomain(null as unknown as string)).toBe('my-business.ohc.app');
  });

  it('converts to lowercase and replaces non-alphanumeric with hyphens', () => {
    expect(generateSubdomain('My Custom Business 123!')).toBe('my-custom-business-123.ohc.app');
    expect(generateSubdomain('Maya\'s Custom Cakes')).toBe('maya-s-custom-cakes.ohc.app');
  });

  it('trims hyphens from the start and end of the cleaned name', () => {
    expect(generateSubdomain('---Test---')).toBe('test.ohc.app');
    expect(generateSubdomain('  Hello World  ')).toBe('hello-world.ohc.app');
  });

  it('returns default when cleaned name is empty', () => {
    expect(generateSubdomain('!!!')).toBe('my-business.ohc.app');
  });
});
