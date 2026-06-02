import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { VideoTutorial } from './VideoTutorial';
import { describe, it, expect } from 'vitest';

describe('VideoTutorial', () => {
  it('renders video details correctly', () => {
    const video = {
      id: 1,
      title: "Sample Video",
      duration: "5:00",
      description: "Sample Description",
      url: "http://example.com/video.mp4"
    };

    render(<VideoTutorial video={video} />);

    expect(screen.getByText('Sample Video')).toBeInTheDocument();
    expect(screen.getByText('5:00')).toBeInTheDocument();
    expect(screen.getByText('Sample Description')).toBeInTheDocument();

    // The video src cannot be directly accessed easily via getByRole if it doesn't have a title,
    // but we can check if a video element is present
    const videoElement = document.querySelector('video');
    expect(videoElement).toBeInTheDocument();
    expect(videoElement?.getAttribute('src')).toBe('http://example.com/video.mp4');
  });
});