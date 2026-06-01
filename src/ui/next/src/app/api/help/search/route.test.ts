import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('/api/help/search GET', () => {
  it('returns all articles when no query is provided', async () => {
    const request = new Request('http://localhost/api/help/search');
    const response = await GET(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.length).toBe(6);
  });

  it('filters articles by title', async () => {
    const request = new Request('http://localhost/api/help/search?q=getting');
    const response = await GET(request);
    const data = await response.json();
    expect(data.length).toBe(2);
    expect(data[0].title).toBe('Getting Started');
    expect(data[1].title).toBe('Getting Paid');
  });

  it('filters articles by description', async () => {
    const request = new Request('http://localhost/api/help/search?q=emails');
    const response = await GET(request);
    const data = await response.json();
    expect(data.length).toBe(1);
    expect(data[0].title).toBe('Finding Customers');
  });

  it('returns empty array when no matches found', async () => {
    const request = new Request('http://localhost/api/help/search?q=xyz123');
    const response = await GET(request);
    const data = await response.json();
    expect(data.length).toBe(0);
  });
});
