import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import '@testing-library/jest-dom';
import { PromoterCard } from '../PromoterCard';

describe('PromoterCard', () => {
    it('renders PromoterCard correctly', () => {
        render(
            <PromoterCard
                productName="Test Product"
                imageUrl="http://example.com/image.jpg"
                draftCopy="Check out this test product!"
                onApprove={() => {}}
                onEdit={() => {}}
            />
        );

        expect(screen.getByText(/The Promoter/)).toBeInTheDocument();
        expect(screen.getByText(/Test Product/)).toBeInTheDocument();
        expect(screen.getByText(/"Check out this test product!"/)).toBeInTheDocument();
    });

    it('handles approve and edit actions', () => {
        const mockApprove = vi.fn();
        const mockEdit = vi.fn();

        render(
            <PromoterCard
                productName="Test Product"
                imageUrl="http://example.com/image.jpg"
                draftCopy="Check out this test product!"
                onApprove={mockApprove}
                onEdit={mockEdit}
            />
        );

        fireEvent.click(screen.getByTestId('promoter-approve-btn'));
        expect(mockApprove).toHaveBeenCalled();

        fireEvent.click(screen.getByTestId('promoter-edit-btn'));
        expect(mockEdit).toHaveBeenCalled();
    });
});
