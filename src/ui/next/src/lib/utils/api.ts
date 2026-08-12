type JsonRequestInit = Readonly<{
  method?: 'GET' | 'PUT' | 'POST' | 'PATCH' | 'DELETE';
  body?: unknown;
}>;

async function requestJson<T>(url: string, init: JsonRequestInit = {}): Promise<T> {
  const response = await fetch(url, {
    method: init.method ?? 'GET',
    headers: init.body === undefined ? undefined : { 'content-type': 'application/json' },
    body: init.body === undefined ? undefined : JSON.stringify(init.body),
    credentials: 'same-origin',
    cache: 'no-store',
  });

  const contentType = response.headers.get('content-type') ?? '';
  const payload = contentType.includes('application/json')
    ? await response.json()
    : await response.text();

  if (!response.ok) {
    const message = typeof payload === 'object' && payload !== null && 'error' in payload
      ? String((payload as { error: unknown }).error)
      : `Request failed with HTTP ${response.status}`;
    throw new Error(message);
  }

  return payload as T;
}

export function fetchJson<T = unknown>(url: string): Promise<T> {
  return requestJson<T>(url);
}

export function putJson<T = unknown>(url: string, body: unknown): Promise<T> {
  return requestJson<T>(url, { method: 'PUT', body });
}
