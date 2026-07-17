import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralGrowthWidget from './ViralGrowthWidget';

describe('ViralGrowthWidget', () => {
    beforeEach(() => {
        vi.useFakeTimers();
        Object.assign(navigator, {
            clipboard: {
                writeText: vi.fn(),
            },
        });
    });

    afterEach(() => {
        vi.useRealTimers();
        vi.restoreAllMocks();
    });

    it('renders correctly with default props', () => {
        render(<ViralGrowthWidget />);

        expect(screen.getByText('Invite Your Network')).toBeInTheDocument();
        expect(screen.getByText('https://ohc.app/join/ohc')).toBeInTheDocument();
        expect(screen.getByRole('button', { name: 'Copy Link' })).toBeInTheDocument();
        expect(screen.getByRole('link', { name: /post/i })).toHaveAttribute('href', expect.stringContaining('https://twitter.com/intent/tweet'));
        expect(screen.getByRole('link', { name: /share/i })).toHaveAttribute('href', expect.stringContaining('https://wa.me/'));
    });

    it('renders correctly with custom tenantId', () => {
        render(<ViralGrowthWidget tenantId="my-custom-store" />);
        expect(screen.getByText('https://ohc.app/join/my-custom-store')).toBeInTheDocument();
    });

    it('copies to clipboard and shows copied state', async () => {
        render(<ViralGrowthWidget tenantId="test-store" />);

        const copyButton = screen.getByRole('button', { name: 'Copy Link' });

        await act(async () => {
            fireEvent.click(copyButton);
        });

        expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.app/join/test-store');
        expect(screen.getByRole('button', { name: 'Copied!' })).toBeInTheDocument();

        await act(async () => {
            vi.advanceTimersByTime(2500);
        });

        expect(screen.getByRole('button', { name: 'Copy Link' })).toBeInTheDocument();
    });
});
