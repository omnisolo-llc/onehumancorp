import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget16 from '../MemoryWidget16';

describe('MemoryWidget16', () => {
    it('renders correctly', () => {
        render(<MemoryWidget16 id="test-16" value={5.5} />);
        expect(screen.getByText('Widget 16 - test-16')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
