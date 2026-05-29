import { Metadata, ResolvingMetadata } from 'next';
import { redirect } from 'next/navigation';

type Props = {
  searchParams: { [key: string]: string | string[] | undefined }
}

export async function generateMetadata(
  { searchParams }: Props,
  parent: ResolvingMetadata
): Promise<Metadata> {
  const title = typeof searchParams.title === 'string' ? searchParams.title : 'One Human Corp';
  const description = typeof searchParams.description === 'string' ? searchParams.description : 'Launch your business online instantly with OHC!';
  const image = typeof searchParams.image === 'string' ? searchParams.image : 'https://ohc.store/default-share.png';
  const urlParam = typeof searchParams.url === 'string' ? searchParams.url : 'https://ohc.store';

  return {
    title,
    description,
    openGraph: {
      title,
      description,
      images: [image],
      url: urlParam,
      type: 'website',
    },
    twitter: {
      card: 'summary_large_image',
      title,
      description,
      images: [image],
    },
  };
}

export default function ShareCardPage({ searchParams }: Props) {
  const urlParam = typeof searchParams.url === 'string' ? searchParams.url : 'https://ohc.store';

  // Basic validation to prevent arbitrary open redirects.
  // It should ideally only redirect to relative paths or trusted domains.
  let targetUrl = 'https://ohc.store';
  try {
    const parsedUrl = new URL(urlParam, 'https://ohc.store');
    if (parsedUrl.hostname === 'ohc.store' || parsedUrl.hostname === 'ohc.app' || urlParam.startsWith('ohc://')) {
        // Use parsedUrl.href to ensure the URL is properly formatted and escaped
        targetUrl = parsedUrl.href;
    }
  } catch (e) {
    // If parsing fails, stick to default
  }

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