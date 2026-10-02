import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { VoiceAssistantFAB } from './VoiceAssistantFAB';
const enqueue = vi.hoisted(() => vi.fn());
vi.mock('../../lib/sync/SyncManager', () => ({ syncManager: { enqueueMutation: enqueue } }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</> }));
let stopped: ReturnType<typeof vi.fn>;
beforeEach(() => {
  enqueue.mockReset(); stopped = vi.fn();
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  Object.defineProperty(navigator, 'mediaDevices', { configurable: true, value: { getUserMedia: vi.fn(async () => ({ getTracks: () => [{ stop: stopped }] })) } });
  vi.stubGlobal('MediaRecorder', class {
    state = 'inactive';
    onstop?: () => void;
    ondataavailable?: (event: { data: Blob }) => void;
    start() { this.state = 'recording'; }
    stop() { this.state = 'inactive'; this.ondataavailable?.({ data: new Blob(['recorded audio']) }); this.onstop?.(); }
  });
  vi.spyOn(console, 'error').mockImplementation(() => {});
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
it('does not claim an offline recording is queued until durable persistence commits', async () => {
  let resolve!: () => void;
  enqueue.mockReturnValue(new Promise<void>(done => { resolve = done; }));
  render(<VoiceAssistantFAB />);
  fireEvent.mouseDown(screen.getByRole('button', { name: 'Voice Assistant' }));
  await screen.findByText('Listening...');
  fireEvent.mouseUp(screen.getByRole('button', { name: 'Voice Assistant' }));
  await waitFor(() => expect(enqueue).toHaveBeenCalledOnce());
  expect(screen.queryByText(/Queued for Sync/)).toBeNull();
  expect(screen.getByText('Processing command...')).toBeVisible();
  await act(async () => resolve());
  expect(await screen.findByText(/Queued for Sync/)).toBeVisible();
  expect(stopped).toHaveBeenCalledOnce();
});
it('reports queue persistence failure without a false success and releases the microphone', async () => {
  enqueue.mockRejectedValue(new Error('Storage unavailable'));
  render(<VoiceAssistantFAB />);
  fireEvent.mouseDown(screen.getByRole('button', { name: 'Voice Assistant' }));
  await screen.findByText('Listening...');
  fireEvent.mouseUp(screen.getByRole('button', { name: 'Voice Assistant' }));
  expect(await screen.findByText('Error. Try again.')).toBeVisible();
  expect(screen.queryByText(/Queued for Sync/)).toBeNull();
  expect(stopped).toHaveBeenCalledOnce();
});
