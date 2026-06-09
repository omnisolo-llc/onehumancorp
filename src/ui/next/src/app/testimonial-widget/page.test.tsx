import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import TestimonialWidgetGenerator from './page';

describe('Testimonial Widget Generator', () => {
    beforeEach(() => {
        Object.assign(navigator, {
            clipboard: {
                writeText: vi.fn().mockImplementation(() => Promise.resolve()),
            },
        });
    });

    it('renders the page correctly with default settings', () => {
        render(<TestimonialWidgetGenerator />);

        expect(screen.getByText('Testimonial Widget 🌟')).toBeDefined();
        expect(screen.getByText('Widget Settings')).toBeDefined();

        const tenantInput = screen.getByDisplayValue('my-business');
        expect(tenantInput).toBeDefined();

        const authorInput = screen.getByDisplayValue('Jane Doe');
        expect(authorInput).toBeDefined();
    });

    it('updates live preview URL when settings change', () => {
        const { container } = render(<TestimonialWidgetGenerator />);

        const authorInput = screen.getByDisplayValue('Jane Doe');
        fireEvent.change(authorInput, { target: { value: 'Alice Smith' } });

        expect(screen.getByDisplayValue('Alice Smith')).toBeDefined();

        const getCodeButton = screen.getByText('Get Widget Code');
        fireEvent.click(getCodeButton);
        const textarea = container.querySelector('textarea[readonly]') as HTMLTextAreaElement;
        expect(textarea.value).toContain('authorName=Alice%20Smith');
    });

    it('opens modal and copies code', async () => {
        render(<TestimonialWidgetGenerator />);

        const getCodeButton = screen.getByText('Get Widget Code');
        fireEvent.click(getCodeButton);

        expect(screen.getByText('Embed Testimonial')).toBeDefined();

        const copyButton = screen.getByText('Copy Code');
        fireEvent.click(copyButton);

        expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
            expect.stringContaining('<iframe src="https://ohc.app/api/v1/growth/testimonial/embed')
        );
        expect(screen.getByText('Copied!')).toBeDefined();

        const closeButton = screen.getByText('Close');
        fireEvent.click(closeButton);
        expect(screen.queryByText('Embed Testimonial')).toBeNull();
    });
});