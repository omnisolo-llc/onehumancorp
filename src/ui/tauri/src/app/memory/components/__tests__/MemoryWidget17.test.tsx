import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget17 from '../MemoryWidget17';

describe('MemoryWidget17', () => {
    it('renders correctly', () => {
        render(<MemoryWidget17 id="test-17" value={5.5} />);
        expect(screen.getByText('Widget 17 - test-17')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
