import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/videos GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches videos from the backend and returns them', async () => {
    const mockVideos = [
      { id: 1, title: 'How to add a product', duration: '1:20' },
      { id: 2, title: 'Setting up payments', duration: '1:15' },
    ];

    const request = new NextRequest('http://localhost:3000/api/videos');
    const response = await GET(request);

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(data).toEqual(mockVideos);
  });
});
