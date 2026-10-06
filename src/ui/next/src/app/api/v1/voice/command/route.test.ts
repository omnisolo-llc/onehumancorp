import { describe, expect, it } from 'vitest';
import { POST } from './route';

describe('Voice Command API Route', () => {
  it('returns a mock transcription for a valid request', async () => {
    const formData = new FormData();
    formData.append('audio', new Blob(['fake audio data'], { type: 'audio/webm' }), 'command.webm');

    const request = new Request('http://localhost:3000/api/v1/voice/command', {
      method: 'POST',
      body: formData,
    });

    const response = await POST(request);
    expect(response.status).toBe(200);

    const data = await response.json();
    expect(data.transcription).toBe('Send a quote for $500 to John for the plumbing fix');
    expect(data.intent).toBe('send_quote');
    expect(data.entities).toEqual({
      amount: 500,
      customer: 'John',
      service: 'plumbing fix'
    });
  });

  it('returns 400 if no audio is provided', async () => {
    const formData = new FormData();

    const request = new Request('http://localhost:3000/api/v1/voice/command', {
      method: 'POST',
      body: formData,
    });

    const response = await POST(request);
    expect(response.status).toBe(400);

    const data = await response.json();
    expect(data.error).toBe('No audio provided');
  });
});
