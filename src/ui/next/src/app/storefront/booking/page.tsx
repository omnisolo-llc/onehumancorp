"use client";

import { useState } from "react";
import { AppShell } from "../../components/AppShell";

export default function StorefrontBooking() {
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<any>(null);

  const handleQuoteRequest = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      const tenantId = typeof window !== 'undefined' ? (localStorage.getItem('tenant_id') || 'default') : 'default';
      const token = typeof window !== 'undefined' ? (localStorage.getItem('token') || '') : '';

      const res = await fetch(`/api/ui/booking/quote`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${token}`
        },
        body: JSON.stringify({
            tenant_id: tenantId,
            customer_id: "customer_123",
            product_id: "prod_123",
            image_data: "fake_base64_image_data",
            problem_description: "A pipe under the sink is leaking"
        })
      });

      if (!res.ok) {
          throw new Error("Failed to get quote");
      }

      const data = await res.json();

      setResult({
        success: true,
        quoteRange: data.quote_range,
        description: data.description,
        depositLink: data.deposit_stripe_link,
        slots: data.available_slots.map((s: any) => s.start_time)
      });
    } catch (err) {
      console.error(err);
      alert("Failed to request quote");
    } finally {
      setLoading(false);
    }
  };

  return (
    <AppShell title="Zero-Touch Service Booking & Quoting">
      <div className="max-w-2xl mx-auto mt-6 bg-white p-6 rounded-lg shadow-sm border border-gray-200">
        <h2 className="text-xl font-bold mb-4">Request a Quote & Book</h2>
        <p className="text-gray-600 mb-6">Upload a photo of your issue (e.g., a broken pipe) and our system will generate a preliminary quote and offer booking times.</p>

        {!result ? (
          <form onSubmit={handleQuoteRequest} className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700">Issue Photo</label>
              <input type="file" accept="image/*" required className="mt-1 block w-full border border-gray-300 rounded-md p-2" />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700">Describe the problem</label>
              <textarea required className="mt-1 block w-full border border-gray-300 rounded-md p-2" rows={3}></textarea>
            </div>
            <button type="submit" disabled={loading} className="app-btn-primary w-full">
              {loading ? "Analyzing image & quoting..." : "Get Quote & Available Times"}
            </button>
          </form>
        ) : (
          <div className="space-y-6">
            <div className="bg-blue-50 border border-blue-200 text-blue-800 p-4 rounded-md">
              <h3 className="font-bold text-lg">Preliminary Quote: {result.quoteRange}</h3>
              <p className="mt-1">{result.description}</p>
            </div>

            <div>
              <h4 className="font-semibold mb-3">Available Times</h4>
              <div className="grid grid-cols-1 gap-2">
                {result.slots.map((s: string) => (
                  <button key={s} className="border border-gray-300 rounded-md p-3 text-left hover:bg-gray-50 flex justify-between items-center">
                    <span>{s}</span>
                    <span className="text-sm bg-gray-100 px-2 py-1 rounded">Select</span>
                  </button>
                ))}
              </div>
            </div>

            <div className="border-t pt-4">
              <p className="text-sm text-gray-600 mb-3">A $50 deposit is required to secure your booking.</p>
              <a href={result.depositLink} target="_blank" rel="noreferrer" className="block text-center app-btn-primary w-full">
                Pay $50 Deposit via Stripe
              </a>
            </div>
          </div>
        )}
      </div>
    </AppShell>
  );
}
