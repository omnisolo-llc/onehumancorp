import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import React from 'react';
import { InteractiveWalkthrough } from './Walkthrough';

describe('InteractiveWalkthrough Component', () => {
    it('renders nothing when isOpen is false', () => {
        const { container } = render(
            <InteractiveWalkthrough
                steps={[]}
                isOpen={false}
                onClose={vi.fn()}
                onComplete={vi.fn()}
            />
        );
        expect(container.firstChild).toBeNull();
    });
});
