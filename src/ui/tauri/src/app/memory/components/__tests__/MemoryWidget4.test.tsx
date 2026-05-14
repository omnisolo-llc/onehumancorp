import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget4 from '../MemoryWidget4';

describe('MemoryWidget4', () => {
    it('renders correctly', () => {
        render(<MemoryWidget4 id="test-4" value={5.5} />);
        expect(screen.getByText('Widget 4 - test-4')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
