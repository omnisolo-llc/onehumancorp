import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget33 from '../MemoryWidget33';

describe('MemoryWidget33', () => {
    it('renders correctly', () => {
        render(<MemoryWidget33 id="test-33" value={5.5} />);
        expect(screen.getByText('Widget 33 - test-33')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
