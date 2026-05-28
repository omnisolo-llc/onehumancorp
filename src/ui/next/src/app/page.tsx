'use client';
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function Home() {
  const router = useRouter();
  const [downloading, setDownloading] = useState<string | null>(null);

  const handleDownload = async (os: string) => {
    setDownloading(os);
    try {
      await fetch('/api/v1/growth/downloads', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          os: os,
          version: '1.0.0', // default version for now
        }),
      });
      // In a real app, this would trigger a file download or redirect to an app store.
      // For now, we'll just simulate it.
      setTimeout(() => setDownloading(null), 1500);
    } catch (e) {
      console.error('Failed to track download', e);
      setDownloading(null);
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] font-inter flex flex-col items-center justify-center p-6 relative overflow-hidden">
      {/* Decorative background elements matching the Glassmorphism tokens */}
      <div className="absolute top-0 right-0 w-96 h-96 bg-indigo-100 rounded-bl-full -z-10 opacity-60 mix-blend-multiply filter blur-3xl"></div>
      <div className="absolute bottom-0 left-0 w-96 h-96 bg-blue-100 rounded-tr-full -z-10 opacity-60 mix-blend-multiply filter blur-3xl"></div>

      <div className="max-w-3xl w-full" style={{
          backdropFilter: 'blur(20px) saturate(200%)',
          background: 'rgba(255, 255, 255, 0.7)',
          border: '1px solid rgba(255, 255, 255, 0.5)',
          borderRadius: '16px',
          padding: '48px',
          boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)'
      }}>
        <div className="text-center mb-12">
          <div className="inline-block p-3 bg-white rounded-2xl shadow-sm mb-6 border border-gray-100">
            <span className="text-4xl">🚀</span>
          </div>
          <h1 className="text-5xl font-bold font-outfit text-gray-900 mb-6 tracking-tight">
            The Hybrid Agentic OS
          </h1>
          <p className="text-xl text-gray-600 leading-relaxed max-w-2xl mx-auto">
            Run your AI swarm securely. Experience unparalleled <strong>Local-First</strong> autonomy with zero data leakage. Switch seamlessly between Cloud and Standalone modes.
          </p>
        </div>

        <div className="flex flex-col sm:flex-row gap-4 justify-center items-center mb-12">
          <button
            onClick={() => handleDownload('mac')}
            disabled={downloading !== null}
            className={`flex items-center gap-3 px-8 py-4 rounded-xl font-bold text-lg transition-all duration-200 ${downloading === 'mac' ? 'bg-gray-100 text-gray-500' : 'bg-white text-gray-900 hover:bg-gray-50 shadow-sm border border-gray-200 hover:shadow-md hover:-translate-y-0.5'}`}
          >
            <svg className="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
               <path d="M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2zm3.176 13.923c-1.528.214-3.125.074-4.502-.455-1.545-.595-2.738-1.748-3.41-3.238C6.602 10.742 6.74 9.11 7.42 7.784c.677-1.32 1.83-2.316 3.235-2.766 1.488-.474 3.093-.34 4.494.348.835.412 1.467 1.05 1.905 1.79.232.392.124.908-.247 1.168-.372.26-.882.164-1.127-.22-.303-.498-.755-.91-1.32-1.186-.965-.472-2.072-.56-3.097-.234-.967.31-1.76.993-2.227 1.898-.466.906-.56 1.996-.217 3.033.46 1.026 1.282 1.815 2.342 2.226.947.363 2.046.46 3.1.312.443-.062.853.255.915.698.062.44-.255.852-.702.914z"/>
            </svg>
            {downloading === 'mac' ? 'Starting...' : 'Download for Mac'}
          </button>

          <button
            onClick={() => handleDownload('windows')}
            disabled={downloading !== null}
            className={`flex items-center gap-3 px-8 py-4 rounded-xl font-bold text-lg transition-all duration-200 ${downloading === 'windows' ? 'bg-gray-100 text-gray-500' : 'bg-blue-600 text-white hover:bg-blue-700 shadow-md hover:shadow-lg hover:-translate-y-0.5'}`}
          >
            <svg className="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
              <path d="M2.5 12a9.5 9.5 0 1 1 19 0 9.5 9.5 0 0 1-19 0zm10-5.5v5h5v-5h-5zm-6 0v5h5v-5h-5zm0 6v5h5v-5h-5zm6 0v5h5v-5h-5z"/>
            </svg>
            {downloading === 'windows' ? 'Starting...' : 'Download for Windows'}
          </button>

          <button
            onClick={() => handleDownload('linux')}
            disabled={downloading !== null}
            className={`flex items-center gap-3 px-8 py-4 rounded-xl font-bold text-lg transition-all duration-200 ${downloading === 'linux' ? 'bg-gray-100 text-gray-500' : 'bg-white text-gray-900 hover:bg-gray-50 shadow-sm border border-gray-200 hover:shadow-md hover:-translate-y-0.5'}`}
          >
            <svg className="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm0-14c-3.31 0-6 2.69-6 6s2.69 6 6 6 6-2.69 6-6-2.69-6-6-6zm0 10c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4z"/>
            </svg>
            {downloading === 'linux' ? 'Starting...' : 'Download for Linux'}
          </button>
        </div>

        <div className="text-center pt-8 border-t border-gray-200/50">
          <p className="text-sm text-gray-500 mb-4">Already have an account?</p>
          <div className="flex justify-center gap-4">
             <Link href="/login" className="text-indigo-600 hover:text-indigo-800 font-semibold transition-colors">
               Sign In to Cloud Mode
             </Link>
             <span className="text-gray-300">•</span>
             <Link href="/dashboard" className="text-gray-600 hover:text-gray-900 font-semibold transition-colors">
               Go to Dashboard (Browser)
             </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
