import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { VoiceAssistant } from './VoiceAssistant';
import { TooltipProvider } from './TooltipRegistry';
import '@testing-library/jest-dom';

const mockStart = vi.fn();
const mockStop = vi.fn();
const mockGetTracks = vi.fn().mockReturnValue([{ stop: vi.fn() }]);

class MockMediaRecorder {
  start = mockStart;
  stop = mockStop;
  ondataavailable: any = null;
  onstop: any = null;
  stream = { getTracks: mockGetTracks };
  constructor(stream: any) {
    this.stream = stream;
  }
}

describe('VoiceAssistant', () => {
  let originalMediaRecorder: any;
  let originalMediaDevices: any;
  let originalFetch: any;

  beforeEach(() => {
    vi.clearAllMocks();
    originalMediaRecorder = (global as any).MediaRecorder;
    (global as any).MediaRecorder = MockMediaRecorder;

    originalMediaDevices = global.navigator.mediaDevices;
    Object.defineProperty(global.navigator, 'mediaDevices', {
      configurable: true,
      value: {
        getUserMedia: vi.fn().mockResolvedValue({ getTracks: mockGetTracks })
      }
    });

    originalFetch = global.fetch;
    global.fetch = vi.fn().mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({}),
        text: () => Promise.resolve("")
      })
    );
  });

  afterEach(() => {
    (global as any).MediaRecorder = originalMediaRecorder;
    Object.defineProperty(global.navigator, 'mediaDevices', {
      configurable: true,
      value: originalMediaDevices
    });
    global.fetch = originalFetch;
  });

  const renderWithProvider = (ui: React.ReactElement) => {
    return render(
      <TooltipProvider>
        {ui}
      </TooltipProvider>
    );
  };

  it('renders correctly', async () => {
    const { container } = renderWithProvider(<VoiceAssistant />);
    const button = await screen.findByRole('button', { name: 'Voice Assistant' });
    expect(button).toBeInTheDocument();
    expect(button).toHaveAttribute('data-voice-assistant-surface', 'trigger');
    expect(container.querySelector('[data-voice-assistant-root]')).toHaveClass('sm:fixed');
    expect(container.querySelector('[data-voice-assistant-root]')).not.toHaveClass('fixed');
    button.focus();
    expect(button).toHaveFocus();
  });

  it('handles microphone mousedown to start recording', async () => {
    renderWithProvider(<VoiceAssistant />);
    const button = await screen.findByRole('button');

    fireEvent.mouseDown(button);

    await waitFor(() => {
      expect(global.navigator.mediaDevices.getUserMedia).toHaveBeenCalled();
      expect(mockStart).toHaveBeenCalled();
      expect(screen.getByText('Listening...').closest('[data-voice-assistant-surface="status"]'))
        .toHaveAttribute('data-voice-assistant-state', 'listening');
    });
  });

  it('handles microphone mouseup to stop recording', async () => {
    renderWithProvider(<VoiceAssistant />);
    const button = await screen.findByRole('button');

    fireEvent.mouseDown(button);

    await waitFor(() => {
      expect(mockStart).toHaveBeenCalled();
    });

    fireEvent.mouseUp(button);

    await waitFor(() => {
        expect(mockStop).toHaveBeenCalled();
    });
  });
});
