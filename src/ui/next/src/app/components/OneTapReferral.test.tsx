import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { expect, test, vi } from 'vitest';
import { OneTapReferral } from './OneTapReferral';

test('renders OneTapReferral and handles copy', async () => {
    // Mock navigator.clipboard
    const mockWriteText = vi.fn();
    Object.assign(navigator, {
        clipboard: {
            writeText: mockWriteText,
        },
    });

    render(<OneTapReferral tenantId="test-tenant" source="dashboard" />);

    // Check if component renders text correctly
    expect(screen.getByText('Refer & Earn $50')).toBeInTheDocument();

    const copyButton = screen.getByRole('button', { name: /copy link/i });
    expect(copyButton).toBeInTheDocument();

    // Click the copy button
    fireEvent.click(copyButton);

    // Check if writeText was called with correct URL
    expect(mockWriteText).toHaveBeenCalledWith('/onboarding?ref=test-tenant&source=dashboard');

    // Check if button text changes to Copied!
    expect(await screen.findByText('Copied!')).toBeInTheDocument();
});
