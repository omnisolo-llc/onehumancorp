import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import VideoTutorialsPage from './page';

vi.mock('../../../components/VideoTutorialList', () => ({
  VideoTutorialList: () => <div data-testid="mock-video-tutorial-list">Mock Video Tutorial List</div>
}));

describe('VideoTutorialsPage', () => {
  it('should render correctly with title, link and video list', () => {
    render(<VideoTutorialsPage />);

    // Verify back link
    const backLink = screen.getByRole('link', { name: /Back to Help Center/i });
    expect(backLink).toBeInTheDocument();
    expect(backLink).toHaveAttribute('href', '/help');

    // Verify Title and Subtitle
    expect(screen.getByText('Video Guides')).toBeInTheDocument();
    expect(screen.getByText('Watch quick, simple tutorials to learn how to manage your store like a pro.')).toBeInTheDocument();

    // Verify component
    expect(screen.getByTestId('mock-video-tutorial-list')).toBeInTheDocument();
  });
});
