import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget24 from '../MemoryWidget24';

describe('MemoryWidget24', () => {
    it('renders correctly', () => {
        render(<MemoryWidget24 id="test-24" value={5.5} />);
        expect(screen.getByText('Widget 24 - test-24')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
