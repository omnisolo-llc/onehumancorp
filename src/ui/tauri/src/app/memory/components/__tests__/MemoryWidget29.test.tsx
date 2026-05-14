import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget29 from '../MemoryWidget29';

describe('MemoryWidget29', () => {
    it('renders correctly', () => {
        render(<MemoryWidget29 id="test-29" value={5.5} />);
        expect(screen.getByText('Widget 29 - test-29')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
