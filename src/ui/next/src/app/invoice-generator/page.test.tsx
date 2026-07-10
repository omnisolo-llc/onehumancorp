import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import InvoiceGeneratorPage from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
    writable: true,
    value: vi.fn().mockImplementation(query => ({
        matches: false,
        media: query,
        onchange: null,
        addListener: vi.fn(), // deprecated
        removeListener: vi.fn(), // deprecated
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
    })),
});

// Mock Next.js router
vi.mock('next/navigation', () => ({
    useRouter: () => ({
        push: vi.fn(),
        prefetch: vi.fn(),
    }),
    useSearchParams: () => new URLSearchParams(),
}));

describe('InvoiceGeneratorPage', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        localStorage.clear();
        localStorage.setItem('tenant', 'test-tenant');
    });

    it('renders the Invoice Generator form correctly', () => {
        render(<InvoiceGeneratorPage />);
        expect(screen.getByText('Create Professional Invoice')).toBeInTheDocument();
        expect(screen.getByLabelText(/Client Name/i)).toBeInTheDocument();
        expect(screen.getByLabelText(/Project Details/i)).toBeInTheDocument();
        expect(screen.getByLabelText(/Amount \(\$\)/i)).toBeInTheDocument();
        expect(screen.getByRole('button', { name: /Generate Shareable Invoice/i })).toBeInTheDocument();
    });

    it('validates form inputs before generating link', () => {
        // Mock alert
        const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
        render(<InvoiceGeneratorPage />);

        fireEvent.click(screen.getByRole('button', { name: /Generate Shareable Invoice/i }));
        expect(alertMock).toHaveBeenCalledWith('Please fill out all fields.');

        alertMock.mockRestore();
    });
});
