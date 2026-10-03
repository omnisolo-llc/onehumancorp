import { Metadata } from 'next';
import RedirectAfterHydration from './RedirectAfterHydration';

type Props = {
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}

const defaultTargetUrl = '/onboarding';
const trustedShareHosts = new Set(['cloud.omnisolo.co', 'omnisolo.co']);

function normalizeShareTarget(rawUrl: string) {
  try {
    // Shared links may name an app path or a canonical HTTPS public host.
    // Credentials, ambiguous separators and nested share redirects are not destinations.
    if (Array.from(rawUrl).some(char => char.charCodeAt(0) < 32 || char.charCodeAt(0) === 127 || char === '\\')) return defaultTargetUrl;
    const parsedUrl = new URL(rawUrl, 'http://localhost:3000');
    if (parsedUrl.username || parsedUrl.password) return defaultTargetUrl;
    const decodedPath = decodeURIComponent(parsedUrl.pathname);
    if (Array.from(decodedPath).some(char => char.charCodeAt(0) < 32 || char.charCodeAt(0) === 127 || char === '\\')) return defaultTargetUrl;
    const targetPath = decodedPath.replace(/\/+/g, '/').replace(/\/$/, '');
    if (targetPath.toLowerCase() === '/share-card') return defaultTargetUrl;
    if (parsedUrl.origin === 'http://localhost:3000') {
      if (parsedUrl.pathname.startsWith('//')) return defaultTargetUrl;
      return `${parsedUrl.pathname}${parsedUrl.search}${parsedUrl.hash}`;
    }
    if (parsedUrl.protocol === 'https:' && !parsedUrl.port && trustedShareHosts.has(parsedUrl.hostname)) {
      return parsedUrl.href;
    }
  } catch {
    // Fall through to the default app route.
  }

  return defaultTargetUrl;
}

export async function generateMetadata(
  { searchParams }: Props
): Promise<Metadata> {
  const resolvedSearchParams = await searchParams;
  const title = typeof resolvedSearchParams.title === 'string' ? resolvedSearchParams.title : 'OmniSolo';
  const description = typeof resolvedSearchParams.description === 'string' ? resolvedSearchParams.description : 'Launch your business online instantly with OmniSolo!';
  const image = typeof resolvedSearchParams.image === 'string' ? resolvedSearchParams.image : undefined;
  const urlParam = typeof resolvedSearchParams.url === 'string' ? resolvedSearchParams.url : defaultTargetUrl;
  const targetUrl = normalizeShareTarget(urlParam);

  return {
    title,
    description,
    openGraph: {
      title,
      description,
      images: image ? [image] : [],
      url: targetUrl,
      type: 'website',
    },
    twitter: {
      card: 'summary_large_image',
      title,
      description,
      images: image ? [image] : [],
    },
  };
}

export default async function ShareCardPage({ searchParams }: Props) {
  const resolvedSearchParams = await searchParams;
  const urlParam = typeof resolvedSearchParams.url === 'string' ? resolvedSearchParams.url : defaultTargetUrl;
  const targetUrl = normalizeShareTarget(urlParam);

  // Metadata stays in the server document for link previews. React escapes the
  // real normalized URL; manually escaping it would corrupt query separators.
  return <RedirectAfterHydration targetUrl={targetUrl} />;
}
