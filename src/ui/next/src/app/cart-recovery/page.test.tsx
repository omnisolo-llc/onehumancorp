import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import CartRecoveryPage from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';

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
        render(<CartRecoveryPage />);
        expect(screen.getByText('Recover Abandoned Carts')).toBeInTheDocument();
        expect(screen.getByText('Generate AI Campaign')).toBeInTheDocument();
    });

    it('toggles auto recovery', () => {
        const { container } = render(<CartRecoveryPage />);
        const toggleBtn = container.querySelector('#auto-recovery-toggle') as HTMLButtonElement;
        fireEvent.click(toggleBtn);
        // Add more specific expectations as needed
    });
});
