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

describe('API tooltips route', () => {
  it('returns a dictionary of tooltips', async () => {
    const response = await GET() as any;
    const data = await response.json();
    expect(response.status).toBe(200);
    expect(typeof data).toBe('object');
    expect(Object.keys(data).length).toBeGreaterThan(0);
    expect(data).toHaveProperty('bio-input-tooltip');
    expect(typeof data['bio-input-tooltip']).toBe('string');
  });
});