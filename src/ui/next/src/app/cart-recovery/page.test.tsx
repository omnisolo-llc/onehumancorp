import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import CartRecoveryPage from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';
import { act } from 'react';

// Mock Next.js router
vi.mock('next/navigation', () => ({
    useRouter: () => ({
        push: vi.fn(),
    }),
}));

// Mock global fetch
global.fetch = vi.fn(() =>
    Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ count: 5 }),
    })
) as any;

describe('CartRecoveryPage', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        localStorage.clear();
        localStorage.setItem('has_pro', 'true');
    });

    it('renders the Cart Recovery page correctly', async () => {
        act(() => { render(<CartRecoveryPage />); });
        expect(screen.getByText('Recover Abandoned Carts')).toBeInTheDocument();
        expect(screen.getByText('Generate AI Campaign')).toBeInTheDocument();
    });

    it('toggles auto recovery', () => {
        act(() => { render(<CartRecoveryPage />); });
        const toggleBtn = document.getElementById('auto-recovery-toggle')!;
        fireEvent.click(toggleBtn);
        // Add more specific expectations as needed
    });
});
