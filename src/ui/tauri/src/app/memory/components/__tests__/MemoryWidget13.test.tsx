import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget13 from '../MemoryWidget13';

describe('MemoryWidget13', () => {
    it('renders correctly', () => {
        render(<MemoryWidget13 id="test-13" value={5.5} />);
        expect(screen.getByText('Widget 13 - test-13')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
