import { EventEmitter } from 'node:events';
import type { CDPSession } from '@playwright/test';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { observeDurablePost } from '../../../../e2e/support/durable_post_receipt';

const origin = 'http://127.0.0.1:3000';
const pathname = '/api/v1/onboarding/launch';
const rawBody = JSON.stringify({ success: true, status: 'launched', preparation_id: 'prepared-one' });
function protocol(result: { body: string; base64Encoded: boolean } = { body: rawBody, base64Encoded: false }) {
    const events = new EventEmitter();
    const commands: { method: string; params: unknown }[] = [];
    const detach = vi.fn(async () => undefined);
    const send = vi.fn(async (method: string, params: unknown) => {
        commands.push({ method, params });
        return method === 'Network.getResponseBody' ? result : {};
    });
    const session = Object.assign(events, { send, detach }) as unknown as CDPSession;
    return { events, commands, detach, send, session };
}
function request(p: ReturnType<typeof protocol>, requestId = 'launch-one', url = origin + pathname, method = 'POST') {
    p.events.emit('Network.requestWillBeSent', { requestId, request: {
        url, method, postData: '{"preparation_id":"prepared-one"}',
        headers: { 'X-OHC-Expected-User': 'user-a', 'X-OHC-Expected-Tenant': 'tenant-a' },
    } });
}
function reply(p: ReturnType<typeof protocol>, requestId = 'launch-one', status = 200, url = origin + pathname) {
    p.events.emit('Network.responseReceived', { requestId, response: { status, url } });
}
function finish(p: ReturnType<typeof protocol>, requestId = 'launch-one', encodedDataLength = rawBody.length) {
    p.events.emit('Network.loadingFinished', { requestId, encodedDataLength });
}
afterEach(() => { vi.useRealTimers(); });

describe('passive durable POST receipt observation', () => {
    it('configures bounded browser-owned storage before returning an action-ready capture', async () => {
        const p = protocol();
        const capture = await observeDurablePost(p.session, origin, pathname);
        expect(p.commands).toEqual([
            { method: 'Network.enable', params: { maxTotalBufferSize: 2097152, maxResourceBufferSize: 262144, maxPostDataSize: 65536 } },
            { method: 'Network.configureDurableMessages', params: { maxTotalBufferSize: 2097152, maxResourceBufferSize: 262144 } },
        ]);
        const rejected = expect(capture.response).rejects.toThrow(/disposed/);
        await capture.dispose(); await rejected;
        expect(p.detach).toHaveBeenCalledTimes(1);
        expect(p.events.eventNames()).toEqual([]);
    });
    it.each([false, true])('retains the actual body and request binding, base64=%s', async base64Encoded => {
        const p = protocol({ body: base64Encoded ? Buffer.from(rawBody).toString('base64') : rawBody, base64Encoded });
        const capture = await observeDurablePost(p.session, origin, pathname);
        request(p); reply(p);
        // Cross-document lifecycle notifications do not substitute a receipt.
        p.events.emit('Page.frameNavigated', { frame: { url: origin + '/dashboard' } });
        finish(p);
        expect(await capture.response).toEqual({ status: 200, body: rawBody,
            requestBody: '{"preparation_id":"prepared-one"}', expectedUser: 'user-a', expectedTenant: 'tenant-a' });
        expect(p.commands.map(command => command.method)).toEqual(['Network.enable', 'Network.configureDurableMessages', 'Network.getResponseBody']);
        expect(p.commands[2].params).toEqual({ requestId: 'launch-one' });
        await capture.dispose();
        expect(p.detach).toHaveBeenCalledTimes(1);
        expect(p.events.eventNames()).toEqual([]);
    });
    it('ignores other origins, paths, methods and response IDs', async () => {
        const p = protocol(); const capture = await observeDurablePost(p.session, origin, pathname);
        request(p, 'foreign', 'https://foreign.invalid' + pathname);
        request(p, 'other-path', origin + '/api/v1/onboarding/start');
        request(p, 'read', origin + pathname, 'GET');
        reply(p, 'foreign'); finish(p, 'foreign');
        request(p); reply(p); finish(p);
        expect((await capture.response).body).toBe(rawBody);
        expect(p.commands.filter(command => command.method === 'Network.getResponseBody')).toHaveLength(1);
    });
    it.each([undefined, 'x'.repeat(65537)])('rejects missing or oversized request bodies', async postData => {
        const p = protocol(); const capture = await observeDurablePost(p.session, origin, pathname);
        p.events.emit('Network.requestWillBeSent', { requestId: 'launch-one', request: { url: origin + pathname, method: 'POST', headers: {}, postData } });
        await expect(capture.response).rejects.toThrow(/POST body/);
        expect(p.detach).toHaveBeenCalledTimes(1);
    });
    it.each(['duplicate', 'redirect', 'wrong-response-url', 'redirect-status', 'loading-failed', 'oversized', 'missing-metadata', 'closed'])('rejects %s and detaches', async cause => {
        const p = protocol(); const capture = await observeDurablePost(p.session, origin, pathname);
        const rejected = expect(capture.response).rejects.toThrow();
        request(p);
        if (cause === 'duplicate') request(p, 'launch-two');
        if (cause === 'redirect') p.events.emit('Network.requestWillBeSent', { requestId: 'launch-one', redirectResponse: {}, request: { url: 'https://foreign.invalid/next', method: 'GET', headers: {} } });
        if (cause === 'wrong-response-url') reply(p, 'launch-one', 200, origin + '/other');
        if (cause === 'redirect-status') reply(p, 'launch-one', 302);
        if (cause === 'loading-failed') p.events.emit('Network.loadingFailed', { requestId: 'launch-one', errorText: 'net::ERR_FAILED' });
        if (cause === 'oversized') { reply(p); finish(p, 'launch-one', 262145); }
        if (cause === 'missing-metadata') finish(p);
        if (cause === 'closed') p.events.emit('close');
        await rejected;
        expect(p.events.eventNames()).toEqual([]);
        expect(p.detach).toHaveBeenCalledTimes(1);
        expect(p.commands.some(command => command.method === 'Network.getResponseBody')).toBe(false);
    });
    it.each([
        { body: '!!!', base64Encoded: true },
        { body: Buffer.from([0xff]).toString('base64'), base64Encoded: true },
        { body: 'x'.repeat(262145), base64Encoded: false },
    ])('rejects invalid or oversized decoded payloads', async result => {
        const p = protocol(result); const capture = await observeDurablePost(p.session, origin, pathname);
        request(p); reply(p); finish(p);
        await expect(capture.response).rejects.toThrow(/body could not be observed/);
        expect(p.detach).toHaveBeenCalledTimes(1);
    });
    it('surfaces unavailable durable protocol support before any action can begin', async () => {
        const p = protocol(); p.send.mockResolvedValueOnce({}).mockRejectedValueOnce(new Error('unknown command'));
        await expect(observeDurablePost(p.session, origin, pathname)).rejects.toThrow(/unavailable/);
        expect(p.send).toHaveBeenCalledTimes(2);
        expect(p.detach).toHaveBeenCalledTimes(1);
        expect(p.events.eventNames()).toEqual([]);
    });
    it('does not return an action-ready capture before durable storage acknowledges configuration', async () => {
        const p = protocol();
        let acknowledge!: (value: Record<string, never>) => void;
        p.send.mockResolvedValueOnce({}).mockImplementationOnce(() => new Promise(resolve => { acknowledge = resolve; }));
        let ready = false;
        const pending = observeDurablePost(p.session, origin, pathname).then(capture => { ready = true; return capture; });
        await Promise.resolve(); await Promise.resolve();
        expect(ready).toBe(false);
        acknowledge({});
        const capture = await pending;
        request(p); reply(p); finish(p);
        expect((await capture.response).body).toBe(rawBody);
    });
    it('surfaces lost actual response-body errors without replay', async () => {
        const p = protocol(); const capture = await observeDurablePost(p.session, origin, pathname);
        p.send.mockRejectedValueOnce(new Error('No resource with given identifier found'));
        request(p); reply(p); finish(p);
        await expect(capture.response).rejects.toThrow(/body could not be observed/);
        expect(p.send).toHaveBeenCalledTimes(3);
        expect(p.detach).toHaveBeenCalledTimes(1);
    });
    it('times out without accepting a pending body and cleans every listener', async () => {
        vi.useFakeTimers();
        const p = protocol(); const capture = await observeDurablePost(p.session, origin, pathname, 50);
        request(p); reply(p);
        const rejected = expect(capture.response).rejects.toThrow(/Timed out/);
        await vi.advanceTimersByTimeAsync(50); await rejected;
        expect(p.events.eventNames()).toEqual([]);
        expect(p.detach).toHaveBeenCalledTimes(1);
    });
    it('retains timeout failure when an already-started body read finishes late', async () => {
        vi.useFakeTimers();
        const p = protocol(); const capture = await observeDurablePost(p.session, origin, pathname, 50);
        let complete!: (value: { body: string; base64Encoded: boolean }) => void;
        p.send.mockImplementationOnce(() => new Promise(resolve => { complete = resolve; }));
        request(p); reply(p); finish(p);
        const rejected = expect(capture.response).rejects.toThrow(/Timed out/);
        await vi.advanceTimersByTimeAsync(50); await rejected;
        complete({ body: rawBody, base64Encoded: false });
        await Promise.resolve();
        await expect(capture.response).rejects.toThrow(/Timed out/);
        expect(p.detach).toHaveBeenCalledTimes(1);
    });
});
