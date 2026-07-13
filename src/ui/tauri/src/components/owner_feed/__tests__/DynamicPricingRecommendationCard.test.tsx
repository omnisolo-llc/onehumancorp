import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { DynamicPricingRecommendationCard } from '../DynamicPricingRecommendationCard';
import '@testing-library/jest-dom';

describe('DynamicPricingRecommendationCard', () => {
    it('renders the recommendation text', () => {
        render(
            <DynamicPricingRecommendationCard
                recommendationText="'Summer Hats' has high stock (50) but low sales. Suggest a 15% discount to clear inventory."
                onApprove={vi.fn()}
                onEdit={vi.fn()}
            />
        );

        expect(screen.getByText(/Summer Hats/)).toBeInTheDocument();
        expect(screen.getByText('✨ AI Advisory')).toBeInTheDocument();
    });

    it('calls onApprove when the approve button is clicked', () => {
        const handleApprove = vi.fn();
        render(
            <DynamicPricingRecommendationCard
                recommendationText="Discount hats"
                onApprove={handleApprove}
                onEdit={vi.fn()}
            />
        );

        fireEvent.click(screen.getByTestId('feed-approve-pricing-btn'));
        expect(handleApprove).toHaveBeenCalledTimes(1);
    });

    it('calls onEdit when the adjust details button is clicked', () => {
        const handleEdit = vi.fn();
        render(
            <DynamicPricingRecommendationCard
                recommendationText="Discount hats"
                onApprove={vi.fn()}
                onEdit={handleEdit}
            />
        );

        fireEvent.click(screen.getByTestId('feed-edit-pricing-btn'));
        expect(handleEdit).toHaveBeenCalledTimes(1);
    });
});
