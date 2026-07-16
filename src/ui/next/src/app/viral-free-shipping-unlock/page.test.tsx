import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralFreeShippingUnlock from './page';

vi.mock('next/navigation', () => ({
    useRouter: vi.fn(() => ({ push: vi.fn() })),
}));

vi.mock('../components/PoweredByOHC', () => ({
    PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralFreeShippingUnlock', () => {
    beforeEach(() => {
        vi.clearAllMocks();

        // Mock localStorage
        const localStorageMock = {
            getItem: vi.fn((key) => {
                if (key === 'tenant_id') return 'mock-tenant';
                if (key === 'has_pro') return 'false';
                return null;
            }),
            setItem: vi.fn(),
            clear: vi.fn()
        };
        Object.defineProperty(window, 'localStorage', {
            value: localStorageMock,
            writable: true
        });

        // Mock clipboard
        Object.assign(navigator, {
            clipboard: {
                writeText: vi.fn(),
            },
        });
    });

    it('renders the initial configuration correctly', async () => {
        await act(async () => {
            render(<ViralFreeShippingUnlock />);
        });

        expect(screen.getByText('Viral Free Shipping Unlock 🚚')).toBeDefined();
        expect(screen.getByText('Widget Settings')).toBeDefined();

        // Check default values
        const minSpendInput = screen.getByDisplayValue('50');
        expect(minSpendInput).toBeDefined();

        const sharesInput = screen.getByDisplayValue('3');
        expect(sharesInput).toBeDefined();

        // Check if embed code exists and has the right default tenant
        const codeElement = screen.getByText((content) => content.includes('mock-tenant'));
        expect(codeElement).toBeDefined();
        expect(codeElement.textContent).toContain('min_spend=50');
        expect(codeElement.textContent).toContain('shares=3');
        expect(codeElement.textContent).toContain('⚡ Powered by OHC');
    });

    it('updates embed code when inputs change', async () => {
        await act(async () => {
            render(<ViralFreeShippingUnlock />);
        });

        const minSpendInput = screen.getByDisplayValue('50');
        const sharesInput = screen.getByDisplayValue('3');

        await act(async () => {
            fireEvent.change(minSpendInput, { target: { value: '100' } });
            fireEvent.change(sharesInput, { target: { value: '5' } });
        });

        const codeElement = screen.getByText((content) => content.includes('mock-tenant'));
        expect(codeElement.textContent).toContain('min_spend=100');
        expect(codeElement.textContent).toContain('shares=5');
    });

    it('shows paywall when non-pro user tries to remove branding', async () => {
        await act(async () => {
            render(<ViralFreeShippingUnlock />);
        });

        const removeBrandingCheckbox = screen.getByRole('checkbox');

        await act(async () => {
            fireEvent.click(removeBrandingCheckbox);
        });

        // The paywall modal should appear
        expect(screen.getByText('Upgrade to Pro')).toBeDefined();
        expect(screen.getByText('White-label your growth widgets. Upgrade to Pro to remove the branding completely and capture 100% of your brand value.') || screen.getByText(/White-label your growth widgets/)).toBeDefined();
    });

    it('copies the embed code to clipboard', async () => {
        await act(async () => {
            render(<ViralFreeShippingUnlock />);
        });

        const copyButton = screen.getByText('Copy Code');

        await act(async () => {
            fireEvent.click(copyButton);
        });

        expect(navigator.clipboard.writeText).toHaveBeenCalled();
        expect(screen.getByText('Copied!')).toBeDefined();
    });

    it('renders the PoweredByOHC footer', async () => {
        await act(async () => {
            render(<ViralFreeShippingUnlock />);
        });

        expect(screen.getByTestId('powered-by-ohc')).toBeDefined();
    });
});
