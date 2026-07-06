import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { VoiceAssistant } from './VoiceAssistant';
import { TooltipProvider } from './TooltipRegistry';
import '@testing-library/jest-dom';

// Mock window.MediaRecorder
const mockMediaRecorderInstance = {
  start: vi.fn(),
  stop: vi.fn(),
  ondataavailable: null,
  onstop: null,
  stream: {
    getTracks: () => [{ stop: vi.fn() }]
  }
};

const MockMediaRecorder = vi.fn().mockImplementation(() => mockMediaRecorderInstance);
(global as any).MediaRecorder = MockMediaRecorder;

global.navigator.mediaDevices = {
  getUserMedia: vi.fn().mockResolvedValue(mockMediaRecorderInstance.stream)
} as any;

global.fetch = vi.fn().mockImplementation(() =>
  Promise.resolve({
    ok: true,
    json: () => Promise.resolve({}),
    text: () => Promise.resolve("")
  })
);

describe('VoiceAssistant', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  const renderWithProvider = (ui: React.ReactElement) => {
    return render(
      <TooltipProvider>
        {ui}
      </TooltipProvider>
    );
  };

  it('renders correctly', async () => {
    renderWithProvider(<VoiceAssistant />);
    const button = await screen.findByRole('button');
    expect(button).toBeInTheDocument();
  });

  it('handles microphone mousedown to start recording', async () => {
    renderWithProvider(<VoiceAssistant />);
    const button = await screen.findByRole('button');

    fireEvent.mouseDown(button);

    await waitFor(() => {
      expect(global.navigator.mediaDevices.getUserMedia).toHaveBeenCalled();
    });

    await waitFor(() => {
        expect(MockMediaRecorder).toHaveBeenCalled();
        expect(mockMediaRecorderInstance.start).toHaveBeenCalled();
    });
  });

  it('handles microphone mouseup to stop recording', async () => {
    renderWithProvider(<VoiceAssistant />);
    const button = await screen.findByRole('button');

    fireEvent.mouseDown(button);

    await waitFor(() => {
      expect(mockMediaRecorderInstance.start).toHaveBeenCalled();
    });

    fireEvent.mouseUp(button);

    await waitFor(() => {
        expect(mockMediaRecorderInstance.stop).toHaveBeenCalled();
    });
  });
});
