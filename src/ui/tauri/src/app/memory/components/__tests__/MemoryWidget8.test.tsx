import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget8 from '../MemoryWidget8';

describe('MemoryWidget8', () => {
    it('renders correctly', () => {
        render(<MemoryWidget8 id="test-8" value={5.5} />);
        expect(screen.getByText('Widget 8 - test-8')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
