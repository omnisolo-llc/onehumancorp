import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('/api/videos GET', () => {
  it('returns the help videos list with specific content', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const data = await response.json();

    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBe(10); // We know there are 10 items

    // Check specific items to ensure content is correct
    const firstVideo = data.find((item: any) => item.id === 1);
    expect(firstVideo).toBeDefined();
    expect(firstVideo.title).toBe('How to set up your first store easily');
    expect(firstVideo.duration).toBe('1:20');

    const lastVideo = data.find((item: any) => item.id === 10);
    expect(lastVideo).toBeDefined();
    expect(lastVideo.title).toBe('Adding staff to your account');
    expect(lastVideo.duration).toBe('0:40');

    // Ensure all items have the required fields
    data.forEach((item: any) => {
      expect(item).toHaveProperty('id');
      expect(typeof item.id).toBe('number');
      expect(item).toHaveProperty('title');
      expect(typeof item.title).toBe('string');
      expect(item).toHaveProperty('duration');
      expect(typeof item.duration).toBe('string');
    });
  });
});
