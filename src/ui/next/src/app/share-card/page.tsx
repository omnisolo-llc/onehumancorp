import { Metadata, ResolvingMetadata } from 'next';

type Props = {
  searchParams: { [key: string]: string | string[] | undefined }
}

const defaultTargetUrl = '/onboarding';
const trustedShareHosts = new Set(['ohc.app', 'onehumancorp.com']);

function normalizeShareTarget(rawUrl: string) {
  try {
    const parsedUrl = new URL(rawUrl, 'http://localhost:3000');
    if (parsedUrl.origin === 'http://localhost:3000') {
      return `${parsedUrl.pathname}${parsedUrl.search}${parsedUrl.hash}`;
    }
    if (trustedShareHosts.has(parsedUrl.hostname)) {
      return parsedUrl.href;
    }
  } catch {
    // Fall through to the default app route.
  }

  return defaultTargetUrl;
}

export async function generateMetadata(
  { searchParams }: Props,
  _parent: ResolvingMetadata
): Promise<Metadata> {
  const title = typeof searchParams.title === 'string' ? searchParams.title : 'One Human Corp';
  const description = typeof searchParams.description === 'string' ? searchParams.description : 'Launch your business online instantly with OHC!';
  const image = typeof searchParams.image === 'string' ? searchParams.image : undefined;
  const urlParam = typeof searchParams.url === 'string' ? searchParams.url : defaultTargetUrl;
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

export default function ShareCardPage({ searchParams }: Props) {
  const urlParam = typeof searchParams.url === 'string' ? searchParams.url : defaultTargetUrl;
  const targetUrl = normalizeShareTarget(urlParam);

  // Sanitize for safe HTML injection
  const safeHtmlTarget = targetUrl.replace(/"/g, '&quot;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

  // Use client-side redirect so crawlers have a chance to read the OG tags
  return (
    <>
      <meta httpEquiv="refresh" content={`0;url=${safeHtmlTarget}`} />
      <script dangerouslySetInnerHTML={{ __html: `window.location.replace(${JSON.stringify(targetUrl).replace(/</g, '\\u003c')});` }} />
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <p className="text-gray-600 font-medium">Redirecting to <a href={safeHtmlTarget} className="text-blue-600 hover:underline">{safeHtmlTarget}</a>...</p>
      </div>
    </>
  );
}
