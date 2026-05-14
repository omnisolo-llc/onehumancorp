import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget7 from '../MemoryWidget7';

describe('MemoryWidget7', () => {
    it('renders correctly', () => {
        render(<MemoryWidget7 id="test-7" value={5.5} />);
        expect(screen.getByText('Widget 7 - test-7')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
