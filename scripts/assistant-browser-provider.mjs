// Test-only measured model boundary. It accepts one exact text-analysis request
// over an owned loopback listener; it has no outgoing network client or tools.
import { createServer } from 'node:http';
import { randomBytes } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';

export const ASSISTANT_PROMPT = 'Analyze the numbers 14 and 28. State their difference using only this text.';
export const ASSISTANT_OUTPUT = 'The difference between 14 and 28 is 14.';
const SYSTEM = 'Analyze only the text submitted by this tenant. No workspace, memory, tools, external actions or command execution are available. State missing information and never claim an action was performed.';
export const ASSISTANT_MAXIMUM = Buffer.byteLength(SYSTEM) + Buffer.byteLength(ASSISTANT_PROMPT) + 4096 + 128 * 2;
export const assistantRequest = () => ({ model: 'owned-browser-model', messages: [
  { role: 'system', content: SYSTEM }, { role: 'user', content: ASSISTANT_PROMPT },
], max_tokens: 128, temperature: 0 });

export async function startAssistantProvider({ runId, tenantId }) {
  if (!/^[a-f0-9]{12}$/.test(runId ?? '') || !/^e2e-growth-[a-f0-9]{8}(?:-[a-f0-9]{4}){3}-[a-f0-9]{12}$/.test(tenantId ?? '')) {
    throw new Error('An owned native run and newly created fixture tenant are required');
  }
  const key = `public-local-assistant-${randomBytes(16).toString('hex')}`;
  const receiptId = `owned-browser-${runId}-${randomBytes(16).toString('hex')}`;
  const requests = [], sockets = new Set();
  let origin, admitted = false, closed = false, signalStarted, signalRelease;
  const started = new Promise(resolve => { signalStarted = resolve; });
  const released = new Promise(resolve => { signalRelease = resolve; });
  const server = createServer(async (request, response) => {
    const row = { method: request.method, path: request.url, body: null, status: null };
    const reply = (status, body) => {
      row.status = status;
      response.writeHead(status, { 'content-type': 'application/json', 'cache-control': 'no-store' });
      response.end(JSON.stringify(body));
    };
    if (requests.length >= 100) return reply(429, { error: 'Owned request limit reached' });
    requests.push(row);
    if (request.headers.host !== new URL(origin).host) return reply(400, { error: 'Exact owned loopback host required' });
    if (request.method !== 'POST' || request.url !== '/v1/chat/completions') return reply(404, { error: 'No other model, tool or provider endpoint is available' });
    if (request.headers.authorization !== `Bearer ${key}`) return reply(401, { error: 'Owned synthetic authentication required' });
    if (!/^application\/json(?:;.*)?$/i.test(request.headers['content-type'] ?? '')) return reply(400, { error: 'JSON required' });
    try {
      const chunks = []; let length = 0;
      for await (const chunk of request) {
        length += chunk.length;
        if (length > 16384) return reply(413, { error: 'Owned request is too large' });
        chunks.push(chunk);
      }
      row.body = JSON.parse(Buffer.concat(chunks).toString('utf8'));
    } catch { return reply(400, { error: 'Invalid owned request body' }); }
    if (!isDeepStrictEqual(row.body, assistantRequest())) return reply(400, { error: 'Request differs from the registered text-only analysis' });
    if (admitted) return reply(409, { error: 'Duplicate model request' });
    admitted = true; signalStarted();
    await released;
    if (closed || response.destroyed) return;
    reply(200, { id: receiptId, choices: [{ message: { role: 'assistant', content: ASSISTANT_OUTPUT }, finish_reason: 'stop' }],
      usage: { prompt_tokens: 100, completion_tokens: 30, total_tokens: 130 } });
  });
  server.requestTimeout = 5000; server.headersTimeout = 5000;
  server.on('connection', socket => { sockets.add(socket); socket.once('close', () => sockets.delete(socket)); });
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  origin = `http://127.0.0.1:${server.address().port}`;
  return {
    receiptId,
    environment: {
      OMNISOLO_LLM_PROVIDER: 'openai-compatible', OMNISOLO_LLM_MODEL: 'owned-browser-model',
      OMNISOLO_LLM_ENDPOINT: `${origin}/v1`, OMNISOLO_LLM_API_KEY: key, OMNISOLO_MAX_TOKENS: '128',
      OMNISOLO_LLM_TENANT_ID: tenantId, OMNISOLO_USAGE_PAYER: 'managed_api', OMNISOLO_USAGE_MAX_REQUEST_MICROS: '20000',
      OMNISOLO_USAGE_RATE_CARDS: JSON.stringify({ 'openai-compatible/owned-browser-model': {
        revision: 'owned-browser-v1', input_micros_per_million: 1000000,
        output_micros_per_million: 2000000, cached_input_micros_per_million: 1000000,
      } }),
    },
    async waitForRequest() {
      let timer;
      try { await Promise.race([started, new Promise((_, reject) => { timer = setTimeout(() => reject(new Error('Owned model request did not arrive')), 15000); })]); }
      finally { clearTimeout(timer); }
    },
    release: () => signalRelease(),
    evidence: () => structuredClone({ requests }),
    async close() {
      if (closed) return; closed = true; signalRelease();
      for (const socket of sockets) socket.destroy();
      server.closeAllConnections();
      await new Promise((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
    },
  };
}
