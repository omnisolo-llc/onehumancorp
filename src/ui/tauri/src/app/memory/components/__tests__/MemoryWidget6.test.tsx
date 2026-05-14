import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget6 from '../MemoryWidget6';

describe('MemoryWidget6', () => {
    it('renders correctly', () => {
        render(<MemoryWidget6 id="test-6" value={5.5} />);
        expect(screen.getByText('Widget 6 - test-6')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
