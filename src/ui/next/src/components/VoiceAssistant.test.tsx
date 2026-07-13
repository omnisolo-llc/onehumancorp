import React from 'react';
import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { VoiceAssistant } from './VoiceAssistant';
import { TooltipProvider } from './TooltipRegistry';
import '@testing-library/jest-dom';

type TestTrack = { stop: ReturnType<typeof vi.fn> };
type TestStream = { getTracks: () => TestTrack[] };

const recorderInstances: MockMediaRecorder[] = [];

class MockMediaRecorder {
  state: 'inactive' | 'recording' = 'inactive';
  ondataavailable: ((event: { data: Blob }) => void) | null = null;
  onstop: (() => void) | null = null;
  start = vi.fn(() => { this.state = 'recording'; });
  stop = vi.fn(() => { this.state = 'inactive'; });

  constructor(public stream: TestStream) {
    recorderInstances.push(this);
  }
}

function createStream(trackCount = 1) {
  const tracks = Array.from({ length: trackCount }, () => ({ stop: vi.fn() }));
  return { stream: { getTracks: () => tracks }, tracks };
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (reason?: unknown) => void;
  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return { promise, resolve, reject };
}

describe('VoiceAssistant', () => {
  let originalMediaRecorder: typeof MediaRecorder;
  let originalMediaDevices: MediaDevices;
  let originalFetch: typeof fetch;
  let getUserMedia: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    vi.clearAllMocks();
    recorderInstances.length = 0;
    originalMediaRecorder = global.MediaRecorder;
    global.MediaRecorder = MockMediaRecorder as unknown as typeof MediaRecorder;

    originalMediaDevices = global.navigator.mediaDevices;
    getUserMedia = vi.fn();
    Object.defineProperty(global.navigator, 'mediaDevices', {
      configurable: true,
      value: { getUserMedia },
    });

    originalFetch = global.fetch;
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ transcription: 'test command' }),
    } as Response);
  });

  afterEach(() => {
    vi.useRealTimers();
    global.MediaRecorder = originalMediaRecorder;
    Object.defineProperty(global.navigator, 'mediaDevices', {
      configurable: true,
      value: originalMediaDevices,
    });
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  const renderVoiceAssistant = () => render(
    <TooltipProvider>
      <VoiceAssistant />
    </TooltipProvider>,
  );

  const voiceCommandFetches = () => vi.mocked(global.fetch).mock.calls
    .filter(([input]) => input === '/api/v1/voice/command');

  async function startWithMouse(stream: TestStream) {
    getUserMedia.mockResolvedValue(stream);
    const result = renderVoiceAssistant();
    const button = screen.getByRole('button');
    fireEvent.mouseDown(button);
    await waitFor(() => expect(recorderInstances).toHaveLength(1));
    return { ...result, button, recorder: recorderInstances[0] };
  }

  it('renders in responsive shell flow with an operable idle state', () => {
    const { container } = renderVoiceAssistant();
    const button = screen.getByRole('button', { name: /voice assistant/i });
    expect(button).toHaveAttribute('data-voice-assistant-surface', 'trigger');
    expect(button).toHaveAttribute('aria-pressed', 'false');
    expect(button).toHaveAccessibleName(/press and hold enter or space/i);
    expect(container.querySelector('[data-voice-assistant-root]')).toHaveClass('sm:fixed');
    expect(container.querySelector('[data-voice-assistant-root]')).not.toHaveClass('fixed');
    button.focus();
    expect(button).toHaveFocus();
  });

  it('releases every track exactly once during a normal mouse stop', async () => {
    const { stream, tracks } = createStream(2);
    const { button, recorder } = await startWithMouse(stream);

    expect(button).toHaveAttribute('aria-pressed', 'true');
    expect(screen.getByRole('status')).toHaveAttribute('aria-live', 'polite');
    fireEvent.mouseUp(button);

    expect(recorder.stop).toHaveBeenCalledTimes(1);
    expect(tracks[0].stop).toHaveBeenCalledTimes(1);
    expect(tracks[1].stop).toHaveBeenCalledTimes(1);

    await act(async () => recorder.onstop?.());
    expect(tracks[0].stop).toHaveBeenCalledTimes(1);
    expect(tracks[1].stop).toHaveBeenCalledTimes(1);
    expect(voiceCommandFetches()).toHaveLength(1);
  });

  it('does not let a stale recorder completion clear a newer recorder', async () => {
    const first = createStream(2);
    const second = createStream(2);
    getUserMedia
      .mockResolvedValueOnce(first.stream)
      .mockResolvedValueOnce(second.stream);
    renderVoiceAssistant();
    const button = screen.getByRole('button');

    fireEvent.mouseDown(button);
    await waitFor(() => expect(recorderInstances).toHaveLength(1));
    const firstRecorder = recorderInstances[0];
    const staleOnStop = firstRecorder.onstop;
    fireEvent.mouseUp(button);
    expect(firstRecorder.stop).toHaveBeenCalledTimes(1);
    first.tracks.forEach((track) => expect(track.stop).toHaveBeenCalledTimes(1));

    fireEvent.mouseDown(button);
    await waitFor(() => expect(recorderInstances).toHaveLength(2));
    const secondRecorder = recorderInstances[1];
    expect(button).toHaveAttribute('aria-pressed', 'true');

    await act(async () => staleOnStop?.());

    expect(button).toHaveAttribute('aria-pressed', 'true');
    expect(secondRecorder.stop).not.toHaveBeenCalled();
    second.tracks.forEach((track) => expect(track.stop).not.toHaveBeenCalled());
    expect(voiceCommandFetches()).toHaveLength(0);

    fireEvent.mouseUp(button);
    expect(secondRecorder.stop).toHaveBeenCalledTimes(1);
    second.tracks.forEach((track) => expect(track.stop).toHaveBeenCalledTimes(1));
    first.tracks.forEach((track) => expect(track.stop).toHaveBeenCalledTimes(1));
  });

  it('stops and releases recording on touch cancellation', async () => {
    const { stream, tracks } = createStream(2);
    getUserMedia.mockResolvedValue(stream);
    renderVoiceAssistant();
    const button = screen.getByRole('button');

    fireEvent.touchStart(button);
    await waitFor(() => expect(recorderInstances).toHaveLength(1));
    expect(button).toHaveAttribute('aria-pressed', 'true');

    fireEvent.touchCancel(button);

    expect(recorderInstances[0].stop).toHaveBeenCalledTimes(1);
    tracks.forEach((track) => expect(track.stop).toHaveBeenCalledTimes(1));
    expect(button).toHaveAttribute('aria-pressed', 'false');
  });

  it('stops recording and suppresses callbacks when unmounted while recording', async () => {
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const { stream, tracks } = createStream(2);
    const { unmount, recorder } = await startWithMouse(stream);
    const retainedDataCallback = recorder.ondataavailable;
    const retainedStopCallback = recorder.onstop;

    unmount();

    expect(recorder.stop).toHaveBeenCalledTimes(1);
    expect(tracks[0].stop).toHaveBeenCalledTimes(1);
    expect(tracks[1].stop).toHaveBeenCalledTimes(1);
    expect(recorder.ondataavailable).toBeNull();
    expect(recorder.onstop).toBeNull();

    await act(async () => {
      retainedDataCallback?.({ data: new Blob(['late audio']) });
      await retainedStopCallback?.();
    });
    expect(voiceCommandFetches()).toHaveLength(0);
    expect(consoleError).not.toHaveBeenCalled();
  });

  it('releases a stream that resolves after unmount without constructing a recorder', async () => {
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const pendingStream = deferred<TestStream>();
    const { stream, tracks } = createStream(2);
    getUserMedia.mockReturnValue(pendingStream.promise);
    const { unmount } = renderVoiceAssistant();

    fireEvent.mouseDown(screen.getByRole('button'));
    expect(getUserMedia).toHaveBeenCalledTimes(1);
    unmount();
    await act(async () => pendingStream.resolve(stream));

    await waitFor(() => expect(tracks[0].stop).toHaveBeenCalledTimes(1));
    expect(tracks[1].stop).toHaveBeenCalledTimes(1);
    expect(recorderInstances).toHaveLength(0);
    expect(voiceCommandFetches()).toHaveLength(0);
    expect(consoleError).not.toHaveBeenCalled();
  });

  it('does not let a stale rejected request cancel a newer recording request', async () => {
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const firstRequest = deferred<TestStream>();
    const secondRequest = deferred<TestStream>();
    const { stream } = createStream();
    getUserMedia
      .mockReturnValueOnce(firstRequest.promise)
      .mockReturnValueOnce(secondRequest.promise);
    renderVoiceAssistant();
    const button = screen.getByRole('button');

    fireEvent.mouseDown(button);
    fireEvent.mouseUp(button);
    fireEvent.mouseDown(button);
    expect(getUserMedia).toHaveBeenCalledTimes(2);

    await act(async () => firstRequest.reject(new Error('stale sensitive detail')));
    await act(async () => secondRequest.resolve(stream));

    await waitFor(() => expect(recorderInstances).toHaveLength(1));
    expect(recorderInstances[0].start).toHaveBeenCalledTimes(1);
    expect(consoleError).not.toHaveBeenCalled();
  });

  it.each([
    ['Enter', 'Enter'],
    ['Space', ' '],
  ])('supports non-repeating %s hold-to-talk semantics', async (_name, key) => {
    const { stream } = createStream();
    getUserMedia.mockResolvedValue(stream);
    renderVoiceAssistant();
    const button = screen.getByRole('button');

    expect(fireEvent.keyDown(button, { key, repeat: false })).toBe(false);
    await waitFor(() => expect(recorderInstances).toHaveLength(1));
    expect(button).toHaveAttribute('aria-pressed', 'true');
    expect(button).toHaveAccessibleName(/listening/i);

    expect(fireEvent.keyDown(button, { key, repeat: true })).toBe(false);
    expect(getUserMedia).toHaveBeenCalledTimes(1);
    expect(fireEvent.keyUp(button, { key })).toBe(false);
    expect(recorderInstances[0].stop).toHaveBeenCalledTimes(1);

    fireEvent.click(button, { detail: 0 });
    expect(getUserMedia).toHaveBeenCalledTimes(1);
  });

  it('supports assistive synthetic-click toggling without pointer double-triggering', async () => {
    const { stream } = createStream();
    getUserMedia.mockResolvedValue(stream);
    renderVoiceAssistant();
    const button = screen.getByRole('button');

    fireEvent.click(button, { detail: 0 });
    await waitFor(() => expect(recorderInstances).toHaveLength(1));
    expect(button).toHaveAttribute('aria-pressed', 'true');

    fireEvent.click(button, { detail: 1 });
    expect(getUserMedia).toHaveBeenCalledTimes(1);
    expect(recorderInstances[0].stop).not.toHaveBeenCalled();

    fireEvent.click(button, { detail: 0 });
    expect(recorderInstances[0].stop).toHaveBeenCalledTimes(1);
    expect(button).toHaveAttribute('aria-pressed', 'false');
  });

  it('keeps pointer hold behavior and announces routine state changes', async () => {
    const { stream } = createStream();
    const { button, recorder } = await startWithMouse(stream);

    const listeningStatus = screen.getByRole('status');
    expect(listeningStatus).toHaveAttribute('data-voice-assistant-state', 'listening');
    expect(listeningStatus).toHaveAttribute('aria-live', 'polite');
    expect(listeningStatus).toHaveAttribute('aria-atomic', 'true');

    fireEvent.mouseUp(button);
    expect(screen.getByRole('status')).toHaveAttribute('data-voice-assistant-state', 'processing');
    await act(async () => recorder.onstop?.());
    await waitFor(() => expect(screen.getByRole('status')).toHaveAttribute('data-voice-assistant-state', 'success'));
  });

  it('announces media errors assertively without logging raw error details', async () => {
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    getUserMedia.mockRejectedValue(new Error('sensitive device detail'));
    renderVoiceAssistant();

    fireEvent.mouseDown(screen.getByRole('button'));

    const status = await screen.findByRole('status');
    expect(status).toHaveAttribute('data-voice-assistant-state', 'error');
    expect(status).toHaveAttribute('aria-live', 'assertive');
    expect(consoleError).toHaveBeenCalledWith('Failed to start voice recording.');
    expect(consoleError).not.toHaveBeenCalledWith(expect.stringContaining('sensitive device detail'));
  });

  it('does not let an old success reset clear a newer listening session', async () => {
    vi.useFakeTimers();
    const first = createStream();
    const second = createStream();
    getUserMedia
      .mockResolvedValueOnce(first.stream)
      .mockResolvedValueOnce(second.stream);
    renderVoiceAssistant();
    const button = screen.getByRole('button');

    await act(async () => {
      fireEvent.mouseDown(button);
      await Promise.resolve();
    });
    const firstRecorder = recorderInstances[0];
    fireEvent.mouseUp(button);
    await act(async () => firstRecorder.onstop?.());
    expect(screen.getByRole('status')).toHaveAttribute('data-voice-assistant-state', 'success');

    await act(async () => {
      fireEvent.mouseDown(button);
      await Promise.resolve();
    });
    expect(screen.getByRole('status')).toHaveAttribute('data-voice-assistant-state', 'listening');
    expect(button).toHaveAttribute('aria-pressed', 'true');

    act(() => vi.advanceTimersByTime(5000));

    expect(screen.getByRole('status')).toHaveAttribute('data-voice-assistant-state', 'listening');
    expect(button).toHaveAttribute('aria-pressed', 'true');
    vi.useRealTimers();
  });

  it('does not let an old error reset clear a newer processing session', async () => {
    vi.useFakeTimers();
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const first = createStream();
    const second = createStream();
    getUserMedia
      .mockResolvedValueOnce(first.stream)
      .mockResolvedValueOnce(second.stream);
    vi.mocked(global.fetch).mockImplementation((input) => Promise.resolve(
      input === '/api/v1/voice/command'
        ? { ok: false, json: () => Promise.resolve({}) } as Response
        : { ok: true, json: () => Promise.resolve({}) } as Response,
    ));
    renderVoiceAssistant();
    const button = screen.getByRole('button');

    await act(async () => {
      fireEvent.mouseDown(button);
      await Promise.resolve();
    });
    const firstRecorder = recorderInstances[0];
    fireEvent.mouseUp(button);
    await act(async () => firstRecorder.onstop?.());
    expect(screen.getByRole('status')).toHaveAttribute('data-voice-assistant-state', 'error');

    await act(async () => {
      fireEvent.mouseDown(button);
      await Promise.resolve();
    });
    fireEvent.mouseUp(button);
    expect(screen.getByRole('status')).toHaveAttribute('data-voice-assistant-state', 'processing');

    act(() => vi.advanceTimersByTime(3000));

    expect(screen.getByRole('status')).toHaveAttribute('data-voice-assistant-state', 'processing');
    expect(button).toHaveAttribute('aria-pressed', 'false');
    expect(consoleError).toHaveBeenCalledWith('Failed to process voice command.');
    vi.useRealTimers();
  });
});
