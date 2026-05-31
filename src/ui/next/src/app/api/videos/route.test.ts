import { GET } from './route';
import { describe, it, expect, vi } from 'vitest';
import { NextResponse } from 'next/server';

vi.mock('next/server', () => {
  return {
    NextResponse: {
      json: (data: any) => ({
        status: 200,
        json: async () => data
      })
    }
  };
});

describe('API videos route', () => {
  it('returns a list of videos', async () => {
    const response = await GET() as any;
    const data = await response.json();
    expect(response.status).toBe(200);
    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBeGreaterThan(0);
    expect(data[0]).toHaveProperty('id');
    expect(data[0]).toHaveProperty('title');
    expect(data[0]).toHaveProperty('duration');
  });
});
