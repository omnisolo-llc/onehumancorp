export default function Pricing() {
  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
      <main className="max-w-6xl w-full p-8 shadow-sm flex flex-col gap-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
        <div className="text-center mb-10">
          <h1 className="text-4xl font-bold font-outfit text-gray-900 mb-4">Simple, transparent pricing</h1>
          <p className="text-xl text-gray-600 font-inter">Choose the plan that fits your business needs.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          {/* Free Tier */}
          <div className="p-6 border border-gray-200 rounded-2xl bg-white shadow-sm flex flex-col justify-between">
            <div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Free</h2>
              <div className="text-3xl font-bold mb-6 text-gray-900">$0<span className="text-lg text-gray-500 font-normal">/mo</span></div>
              <ul className="space-y-3 mb-8 text-gray-600">
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> 100 AI actions / month</li>
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> 500 MB Storage</li>
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> Basic support</li>
              </ul>
            </div>
            <button className="w-full py-2 font-semibold text-blue-600 bg-blue-50 hover:bg-blue-100 transition-colors rounded-lg">Current Plan</button>
          </div>

          {/* Starter Tier */}
          <div className="p-6 border border-gray-200 rounded-2xl bg-white shadow-sm flex flex-col justify-between">
            <div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Starter</h2>
              <div className="text-3xl font-bold mb-6 text-gray-900">$9<span className="text-lg text-gray-500 font-normal">/mo</span></div>
              <ul className="space-y-3 mb-8 text-gray-600">
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> 1,000 AI actions / month</li>
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> 5 GB Storage</li>
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> Priority support</li>
              </ul>
            </div>
            <button className="w-full py-2 font-semibold text-white bg-blue-600 hover:bg-blue-700 transition-colors shadow-sm rounded-lg">Upgrade to Starter</button>
          </div>

          {/* Pro Tier */}
          <div className="p-6 border-2 border-blue-500 rounded-2xl bg-blue-50 shadow-md flex flex-col justify-between relative">
            <div className="absolute top-0 right-0 bg-blue-500 text-white text-xs font-bold px-3 py-1 rounded-bl-lg rounded-tr-lg">POPULAR</div>
            <div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Pro</h2>
              <div className="text-3xl font-bold mb-6 text-gray-900">$29<span className="text-lg text-gray-500 font-normal">/mo</span></div>
              <ul className="space-y-3 mb-8 text-gray-600">
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> 5,000 AI actions / month</li>
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> 20 GB Storage</li>
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> Advanced analytics</li>
              </ul>
            </div>
            <button className="w-full py-2 font-semibold text-white bg-blue-600 hover:bg-blue-700 transition-colors shadow-sm rounded-lg">Upgrade to Pro</button>
          </div>

          {/* Business Tier */}
          <div className="p-6 border border-gray-200 rounded-2xl bg-white shadow-sm flex flex-col justify-between">
            <div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Business</h2>
              <div className="text-3xl font-bold mb-6 text-gray-900">$79<span className="text-lg text-gray-500 font-normal">/mo</span></div>
              <ul className="space-y-3 mb-8 text-gray-600">
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> Unlimited AI actions</li>
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> 100 GB Storage</li>
                <li className="flex items-center gap-2"><span className="text-green-500">✓</span> Dedicated account manager</li>
              </ul>
            </div>
            <button className="w-full py-2 font-semibold text-blue-600 bg-blue-50 hover:bg-blue-100 transition-colors rounded-lg">Upgrade to Business</button>
          </div>
        </div>
      </main>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
