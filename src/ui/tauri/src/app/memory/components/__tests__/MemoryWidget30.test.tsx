import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget30 from '../MemoryWidget30';

describe('MemoryWidget30', () => {
    it('renders correctly', () => {
        render(<MemoryWidget30 id="test-30" value={5.5} />);
        expect(screen.getByText('Widget 30 - test-30')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
