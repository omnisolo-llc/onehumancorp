import React from 'react';

export default function ApiDocsPage() {
  return (
    <div className="min-h-screen bg-slate-50 py-12 px-4 sm:px-6 lg:px-8 font-sans">
      <div className="max-w-4xl mx-auto bg-white rounded-2xl shadow-xl overflow-hidden border border-slate-200">
        <div className="bg-slate-900 px-6 py-8 sm:p-10 border-b border-slate-800">
          <h1 className="text-3xl font-extrabold text-white mb-2" style={{ fontFamily: 'Outfit, sans-serif' }}>API Documentation</h1>
          <p className="text-slate-300 text-sm">Advanced tools for developers to connect custom checkouts and external systems.</p>
        </div>

        <div className="p-6 sm:p-10 space-y-8">
          <section>
            <h2 className="text-xl font-bold text-slate-800 mb-4 border-b border-slate-100 pb-2">Authentication</h2>
            <p className="text-slate-600 text-sm mb-4">All API requests require a Bearer token in the Authorization header. You can generate API keys from your account dashboard.</p>
            <pre className="bg-slate-900 text-slate-200 p-4 rounded-lg text-xs overflow-x-auto">
              <code>Authorization: Bearer sk_live_your_api_key_here</code>
            </pre>
          </section>

          <section>
            <h2 className="text-xl font-bold text-slate-800 mb-4 border-b border-slate-100 pb-2">Endpoints</h2>

            <div className="space-y-6">
              {/* Endpoint 1 */}
              <div className="bg-white border border-slate-200 rounded-xl overflow-hidden">
                <div className="bg-slate-50 px-4 py-3 border-b border-slate-200 flex items-center gap-3">
                  <span className="px-2 py-1 bg-green-100 text-green-700 text-xs font-bold rounded uppercase">GET</span>
                  <code className="text-sm font-semibold text-slate-800">/v1/payments</code>
                </div>
                <div className="p-4">
                  <p className="text-sm text-slate-600 mb-2">List all payments for your account.</p>
                  <h4 className="text-xs font-bold text-slate-800 mt-4 mb-2 uppercase tracking-wide">Response</h4>
                  <pre className="bg-slate-900 text-slate-200 p-3 rounded-lg text-xs overflow-x-auto">
{`{
  "data": [
    {
      "id": "pay_123",
      "amount": 4500,
      "currency": "usd",
      "status": "succeeded"
    }
  ],
  "has_more": false
}`}
                  </pre>
                </div>
              </div>

              {/* Endpoint 2 */}
              <div className="bg-white border border-slate-200 rounded-xl overflow-hidden">
                <div className="bg-slate-50 px-4 py-3 border-b border-slate-200 flex items-center gap-3">
                  <span className="px-2 py-1 bg-blue-100 text-blue-700 text-xs font-bold rounded uppercase">POST</span>
                  <code className="text-sm font-semibold text-slate-800">/v1/payments</code>
                </div>
                <div className="p-4">
                  <p className="text-sm text-slate-600 mb-2">Create a new payment request.</p>
                  <h4 className="text-xs font-bold text-slate-800 mt-4 mb-2 uppercase tracking-wide">Body Parameters</h4>
                  <ul className="text-sm text-slate-600 list-disc list-inside mb-4">
                    <li><code className="bg-slate-100 px-1 rounded text-pink-600">amount</code> (integer, required) - Amount in cents</li>
                    <li><code className="bg-slate-100 px-1 rounded text-pink-600">currency</code> (string, required) - 3-letter ISO code</li>
                  </ul>
                </div>
              </div>
            </div>
          </section>
        </div>
      </div>
    </div>
  );
}
