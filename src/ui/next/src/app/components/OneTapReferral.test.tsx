import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { OneTapReferral } from './OneTapReferral';

describe('OneTapReferral Component', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        Object.assign(navigator, {
            clipboard: {
                writeText: vi.fn(),
            },
        });
    });

    it('renders the component properly', () => {
        render(<OneTapReferral tenantId="test-store" source="test" />);
        expect(screen.getByText('Refer & Earn $50')).toBeDefined();
        expect(screen.getByText('Invite a friend to OHC and you both get rewarded!')).toBeDefined();
        expect(screen.getByText('Copy Link')).toBeDefined();
        expect(screen.getByText('WhatsApp')).toBeDefined();
    });

    it('generates the correct referral link format', () => {
        render(<OneTapReferral tenantId="my-store-123" source="sidebar" />);

        const copyButton = screen.getByText('Copy Link');
        fireEvent.click(copyButton);

        expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
            expect.stringContaining('/r/my-store-123?offer=get_50')
        );
    });

    it('shows Copied! state briefly', async () => {
        vi.useFakeTimers();
        render(<OneTapReferral tenantId="test-store" source="test" />);

        const copyButton = screen.getByText('Copy Link');

        fireEvent.click(copyButton);

        expect(screen.getByText('Copied!')).toBeDefined();

        act(() => {
           vi.advanceTimersByTime(2500);
        });

        expect(screen.getByText('Copy Link')).toBeDefined();

        vi.useRealTimers();
    });
});
