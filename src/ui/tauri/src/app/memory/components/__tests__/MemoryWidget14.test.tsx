import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget14 from '../MemoryWidget14';

describe('MemoryWidget14', () => {
    it('renders correctly', () => {
        render(<MemoryWidget14 id="test-14" value={5.5} />);
        expect(screen.getByText('Widget 14 - test-14')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
