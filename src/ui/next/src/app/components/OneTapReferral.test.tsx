import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import { OneTapReferral } from './OneTapReferral';

beforeEach(() => {
    vi.clearAllMocks();
});

test('renders OneTapReferral and handles copy with fetched link', async () => {
    // Mock navigator.clipboard
    const mockWriteText = vi.fn();
    Object.assign(navigator, {
        clipboard: {
            writeText: mockWriteText,
        },
    });

    // Mock fetch
    global.fetch = vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ referral_link: 'ohc://join?ref=1234abcd' })
    });

    await act(async () => {
        render(<OneTapReferral tenantId="test-tenant" source="dashboard" />);
    });

    // Check if component renders text correctly
    expect(screen.getByText('Refer & Earn $50')).toBeInTheDocument();

    const copyButton = screen.getByRole('button', { name: /copy link/i });
    expect(copyButton).toBeInTheDocument();

    // Click the copy button
    fireEvent.click(copyButton);

    // Check if writeText was called with correct URL
    await waitFor(() => {
        expect(mockWriteText).toHaveBeenCalledWith('ohc://join?ref=1234abcd');
    });

    // Check if button text changes to Copied!
    expect(await screen.findByText('Copied!')).toBeInTheDocument();
});

test('renders OneTapReferral and falls back to default link on fetch failure', async () => {
    // Mock navigator.clipboard
    const mockWriteText = vi.fn();
    Object.assign(navigator, {
        clipboard: {
            writeText: mockWriteText,
        },
    });

    // Mock fetch
    global.fetch = vi.fn().mockRejectedValue(new Error('Failed'));

    await act(async () => {
        render(<OneTapReferral tenantId="test-tenant" source="dashboard" />);
    });

    // Check if component renders text correctly
    expect(screen.getByText('Refer & Earn $50')).toBeInTheDocument();

    const copyButton = screen.getByRole('button', { name: /copy link/i });
    expect(copyButton).toBeInTheDocument();

    // Click the copy button
    fireEvent.click(copyButton);

    // Check if writeText was called with correct default URL
    await waitFor(() => {
        expect(mockWriteText).toHaveBeenCalledWith('/onboarding?ref=test-tenant&source=dashboard');
    });
});
