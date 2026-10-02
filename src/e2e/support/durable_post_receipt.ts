import type { CDPSession, Page } from '@playwright/test';

const totalBytes = 2 * 1024 * 1024;
const receiptBytes = 256 * 1024;
const postBytes = 64 * 1024;
export type DurablePostReceipt = {
    status: number;
    body: string;
    requestBody: string;
    expectedUser: string | undefined;
    expectedTenant: string | undefined;
};
type RequestEvent = {
    requestId: string;
    request: { url: string; method: string; postData?: string; headers: Record<string, string> };
    redirectResponse?: unknown;
};
type ResponseEvent = { requestId: string; response: { url: string; status: number } };

/** Passive Chromium observation; never intercepts, replays or replaces a request. */
export async function captureDurablePost(page: Page, pathname: string) {
    const url = new URL(page.url());
    if (!['http:', 'https:'].includes(url.protocol) || !['127.0.0.1', 'localhost', '[::1]'].includes(url.hostname)) {
        throw new Error('Durable receipt observation requires the actual loopback app.');
    }
    const session = await page.context().newCDPSession(page);
    return observeDurablePost(session, url.origin, pathname);
}

/** Exported for recorded-protocol boundary tests, which are not browser proof. */
export async function observeDurablePost(session: CDPSession, origin: string, pathname: string, timeoutMs = 30_000) {
    const target = new URL(pathname, origin);
    if (target.origin !== origin || target.pathname !== pathname || target.search || target.hash) {
        await session.detach();
        throw new Error('Receipt destination must be an exact same-origin path.');
    }
    let request: RequestEvent | undefined;
    let status: number | undefined;
    let reading = false;
    let settled = false;
    let detached: Promise<void> | undefined;
    let resolve!: (receipt: DurablePostReceipt) => void;
    let reject!: (error: Error) => void;
    const response = new Promise<DurablePostReceipt>((ok, fail) => { resolve = ok; reject = fail; });
    // The UI action can fail before its caller awaits the receipt. Keep the
    // rejection observed while preserving it for that caller.
    void response.catch(() => undefined);
    const timer = setTimeout(() => finish(new Error('Timed out observing the actual POST receipt')), timeoutMs);
    const cleanup = () => {
        clearTimeout(timer);
        session.off('Network.requestWillBeSent', onRequest);
        session.off('Network.responseReceived', onResponse);
        session.off('Network.loadingFinished', onFinished);
        session.off('Network.loadingFailed', onFailed);
        session.off('close', onClose);
        detached ??= session.detach();
        return detached;
    };
    function finish(error?: Error, value?: DurablePostReceipt) {
        if (settled) return;
        settled = true;
        void cleanup().then(() => {
            if (error) reject(error);
            else if (value) resolve(value);
            else reject(new Error('Receipt observation ended without a body'));
        }, cause => reject(error ?? new Error('Receipt observer could not detach', { cause })));
    }
    const matches = (raw: string) => raw === target.href;
    function onRequest(event: RequestEvent) {
        if (request?.requestId === event.requestId && event.redirectResponse) {
            finish(new Error('Observed POST redirected; no receipt accepted'));
            return;
        }
        if (event.request.method !== 'POST' || !matches(event.request.url)) return;
        if (event.redirectResponse || request) {
            finish(new Error('Observed duplicate or redirected POST; no receipt accepted'));
            return;
        }
        if (typeof event.request.postData !== 'string' || Buffer.byteLength(event.request.postData, 'utf8') > postBytes) {
            finish(new Error('Observed POST body is missing or exceeds the receipt bound'));
            return;
        }
        request = event;
    }
    function onResponse(event: ResponseEvent) {
        if (event.requestId !== request?.requestId) return;
        if (!matches(event.response.url) || !Number.isInteger(event.response.status) || event.response.status < 100
            || event.response.status > 599 || (event.response.status >= 300 && event.response.status < 400)) {
            finish(new Error('Observed response is not the exact finite POST destination'));
            return;
        }
        status = event.response.status;
    }
    function onFinished(event: { requestId: string; encodedDataLength: number }) {
        if (event.requestId !== request?.requestId || settled || reading) return;
        if (status === undefined || !Number.isFinite(event.encodedDataLength) || event.encodedDataLength < 0 || event.encodedDataLength > receiptBytes) {
            finish(new Error('Observed receipt is missing response metadata or exceeds the body bound'));
            return;
        }
        reading = true;
        void session.send('Network.getResponseBody', { requestId: event.requestId }).then(result => {
            if (settled) return;
            const bytes = result.base64Encoded ? Buffer.from(result.body, 'base64') : Buffer.from(result.body, 'utf8');
            if (bytes.length > receiptBytes || (result.base64Encoded && bytes.toString('base64') !== result.body)) {
                throw new Error('Observed receipt is oversized or has invalid base64');
            }
            const headers = Object.fromEntries(Object.entries(request!.request.headers).map(([key, value]) => [key.toLowerCase(), value]));
            finish(undefined, {
                status: status!, body: new TextDecoder('utf-8', { fatal: true }).decode(bytes), requestBody: request!.request.postData!,
                expectedUser: headers['x-ohc-expected-user'], expectedTenant: headers['x-ohc-expected-tenant'],
            });
        }).catch(cause => finish(new Error('Actual POST body could not be observed', { cause })));
    }
    function onFailed(event: { requestId: string; errorText: string }) {
        if (event.requestId === request?.requestId) finish(new Error(`Observed POST failed: ${event.errorText}`));
    }
    function onClose() { finish(new Error('Receipt observation target closed')); }
    session.on('Network.requestWillBeSent', onRequest);
    session.on('Network.responseReceived', onResponse);
    session.on('Network.loadingFinished', onFinished);
    session.on('Network.loadingFailed', onFailed);
    session.on('close', onClose);
    try {
        // Supported by the installed protocol. Unsupported Chromium is a visible
        // prerequisite failure, never a fallback to a weaker receipt assertion.
        await session.send('Network.enable', { maxTotalBufferSize: totalBytes, maxResourceBufferSize: receiptBytes, maxPostDataSize: postBytes });
        await session.send('Network.configureDurableMessages', { maxTotalBufferSize: totalBytes, maxResourceBufferSize: receiptBytes });
    } catch (cause) {
        const error = new Error('Chromium durable receipt observation is unavailable', { cause });
        finish(error);
        await cleanup();
        throw error;
    }
    return { response, dispose: async () => { finish(new Error('Receipt observation disposed before completion')); await cleanup(); } };
}
