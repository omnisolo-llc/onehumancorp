import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget27 from '../MemoryWidget27';

describe('MemoryWidget27', () => {
    it('renders correctly', () => {
        render(<MemoryWidget27 id="test-27" value={5.5} />);
        expect(screen.getByText('Widget 27 - test-27')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
