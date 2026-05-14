import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget20 from '../MemoryWidget20';

describe('MemoryWidget20', () => {
    it('renders correctly', () => {
        render(<MemoryWidget20 id="test-20" value={5.5} />);
        expect(screen.getByText('Widget 20 - test-20')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
