import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('/api/help/[articleId] GET', () => {
  it('returns an article if it exists', async () => {
    const request = new Request('http://localhost:3000/api/help/getting-started-1');
    const response = await GET(request, { params: { articleId: 'getting-started-1' } });

    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.title).toBe('Getting Started with Your Store');
    expect(data.contentHtml).toBeDefined();
  });

  it('returns a 404 error if article is not found', async () => {
    const request = new Request('http://localhost:3000/api/help/non-existent-article');
    const response = await GET(request, { params: { articleId: 'non-existent-article' } });

    expect(response.status).toBe(404);
    const data = await response.json();
    expect(data.error).toBe('Article not found');
  });
});
