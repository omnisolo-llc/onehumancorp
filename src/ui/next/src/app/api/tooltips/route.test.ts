import { GET } from './route';
import { describe, it, expect } from 'vitest';
import { NextResponse } from 'next/server';

describe('Tooltips API Route', () => {
    it('returns a successful JSON response with tooltips', async () => {
        const response = await GET();
        expect(response).toBeInstanceOf(NextResponse);
        expect(response.status).toBe(200);

        const data = await response.json();
        expect(data).toHaveProperty('bio-input-tooltip');
        expect(data['bio-input-tooltip']).toBe('Tell us what you sell and who your customers are. Keep it simple!');
        expect(data['generate-btn-tooltip']).toBe('Click here to have our AI build your ready-to-launch store.');
    });
});
