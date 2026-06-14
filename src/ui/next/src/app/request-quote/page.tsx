"use client";
import React, { useState } from 'react';

export default function RequestQuotePage() {
  const [description, setDescription] = useState('');
  const [imageUrl, setImageUrl] = useState('');
  const [status, setStatus] = useState<'idle' | 'submitting' | 'success' | 'error'>('idle');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setStatus('submitting');

    // Fallback tenant & customer values for this demo flow
    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
    const customerId = typeof localStorage !== 'undefined' ? localStorage.getItem('customer_id') || 'guest' : 'guest';

    try {
      const res = await fetch('/api/quotes/intake', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': tenantId,
          'x-user-id': customerId
        },
        body: JSON.stringify({
          description,
          image_url: imageUrl,
          tenant_id: tenantId,
          customer_id: customerId
        })
      });

      if (res.ok) {
        setStatus('success');
      } else {
        setStatus('error');
      }
    } catch (err) {
      console.error(err);
      setStatus('error');
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
      <div className="max-w-md w-full bg-white rounded-2xl shadow-xl overflow-hidden">
        <div className="px-6 py-8 md:p-10 text-center">
          <h1 className="text-3xl font-extrabold text-gray-900 mb-2">Request an Estimate</h1>
          <p className="text-gray-500 text-sm">Provide details and an image of your issue, and we'll send you an itemized quote.</p>
        </div>

        <form onSubmit={handleSubmit} className="px-6 pb-8 md:px-10 md:pb-10">
          <div className="space-y-6">
            <div>
              <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-2">
                Describe the problem or project
              </label>
              <textarea
                id="description"
                rows={4}
                required
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:ring-2 focus:ring-[#0066FF] focus:border-transparent outline-none transition-all resize-none text-gray-900 placeholder-gray-400"
                placeholder="e.g. My kitchen sink is leaking from the pipe underneath..."
              />
            </div>

            <div>
              <label htmlFor="imageUrl" className="block text-sm font-medium text-gray-700 mb-2">
                Photo (URL)
              </label>
              <input
                id="imageUrl"
                type="url"
                value={imageUrl}
                onChange={(e) => setImageUrl(e.target.value)}
                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:ring-2 focus:ring-[#0066FF] focus:border-transparent outline-none transition-all text-gray-900 placeholder-gray-400"
                placeholder="https://example.com/photo.jpg"
              />
            </div>

            {status === 'error' && (
              <div className="p-4 bg-red-50 text-red-700 rounded-xl text-sm font-medium text-center">
                Something went wrong. Please try again.
              </div>
            )}

            {status === 'success' ? (
              <div className="p-6 bg-green-50 rounded-xl text-center">
                <div className="text-green-500 text-3xl mb-2">✓</div>
                <h3 className="font-bold text-green-800 text-lg">Request Sent!</h3>
                <p className="text-green-600 text-sm mt-1">We'll review your details and send you a formal quote shortly.</p>
              </div>
            ) : (
              <button
                type="submit"
                disabled={status === 'submitting'}
                className={`w-full py-4 rounded-xl text-white font-bold text-lg shadow-md transition-all flex items-center justify-center ${
                  status === 'submitting'
                    ? 'bg-gray-400 cursor-not-allowed'
                    : 'bg-[#0066FF] hover:bg-[#0052CC] hover:shadow-lg hover:-translate-y-0.5'
                }`}
              >
                {status === 'submitting' ? 'Sending...' : 'Get Estimate'}
              </button>
            )}
          </div>
        </form>
      </div>
    </div>
  );
}
