import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget1 from '../MemoryWidget1';

describe('MemoryWidget1', () => {
    it('renders correctly', () => {
        render(<MemoryWidget1 id="test-1" value={5.5} />);
        expect(screen.getByText('Widget 1 - test-1')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
