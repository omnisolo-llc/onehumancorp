import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget32 from '../MemoryWidget32';

describe('MemoryWidget32', () => {
    it('renders correctly', () => {
        render(<MemoryWidget32 id="test-32" value={5.5} />);
        expect(screen.getByText('Widget 32 - test-32')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
