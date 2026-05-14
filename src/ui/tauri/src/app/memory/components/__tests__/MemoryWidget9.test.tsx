import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget9 from '../MemoryWidget9';

describe('MemoryWidget9', () => {
    it('renders correctly', () => {
        render(<MemoryWidget9 id="test-9" value={5.5} />);
        expect(screen.getByText('Widget 9 - test-9')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
