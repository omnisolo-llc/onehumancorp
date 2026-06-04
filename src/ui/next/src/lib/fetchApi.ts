export async function fetchApi(url: string, options?: RequestInit) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const fullUrl = `${backendUrl}${url}`;

  const response = await fetch(fullUrl, options);
  return response;
}
