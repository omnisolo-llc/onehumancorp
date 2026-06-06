import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import VideoTutorialsPage from './page';
import { describe, it, expect, vi } from 'vitest';

vi.mock('../../../components/VideoTutorialList', () => ({
  VideoTutorialList: () => <div data-testid="video-list-mock">Video List Mock</div>
}));

describe('VideoTutorialsPage', () => {
  it('renders correctly', () => {
    render(<VideoTutorialsPage />);
    expect(screen.getByText('Video Guides')).toBeInTheDocument();
    expect(screen.getByTestId('video-list-mock')).toBeInTheDocument();
  });
});
