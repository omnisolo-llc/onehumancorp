import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('/api/help GET', () => {
  it('returns the help articles list with specific content', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const data = await response.json();

    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBe(6); // We know there are 6 items

    // Check specific items to ensure content is correct
    const gettingStarted = data.find((item: any) => item.link === '/help/getting-started-1');
    expect(gettingStarted).toBeDefined();
    expect(gettingStarted.title).toBe('Getting Started');
    expect(gettingStarted.desc).toContain('easily set up your store');

    const myStore = data.find((item: any) => item.link === '/help/my-store');
    expect(myStore).toBeDefined();
    expect(myStore.title).toBe('My Store');
    expect(myStore.desc).toContain('Add products');

    // Ensure all items have the required fields
    data.forEach((item: any) => {
      expect(item).toHaveProperty('title');
      expect(typeof item.title).toBe('string');
      expect(item).toHaveProperty('desc');
      expect(typeof item.desc).toBe('string');
      expect(item).toHaveProperty('link');
      expect(typeof item.link).toBe('string');
    });
  });
});
