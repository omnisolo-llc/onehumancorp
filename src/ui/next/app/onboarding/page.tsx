"use client";

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function Onboarding() {
  const router = useRouter();
  const [step, setStep] = useState<number | string>(0);
  const [advancedMode, setAdvancedMode] = useState(false);

  // Form State
  const [businessType, setBusinessType] = useState("");
  const [businessName, setBusinessName] = useState("");
  const [productName, setProductName] = useState("");
  const [productPrice, setProductPrice] = useState("");
  const [adminName, setAdminName] = useState("");
  const [adminEmail, setAdminEmail] = useState("");
  const [adminPassword, setAdminPassword] = useState("");

  useEffect(() => {
    fetch('/api/onboarding/state')
      .then(res => res.json())
      .then(data => {
        if (data && data.step !== undefined) {
          setStep(data.step);
          if (data.businessName) setBusinessName(data.businessName);
          if (data.productName) setProductName(data.productName);
          if (data.productPrice) setProductPrice(data.productPrice);
        }
      })
      .catch(console.error);
  }, []);

  const saveState = (nextStep: number | string) => {
    fetch('/api/onboarding/state', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ step: nextStep, businessName, productName, productPrice, businessType, adminName, adminEmail })
    }).catch(console.error);
  };

  const saveAndNext = (nextStep: number | string) => {
    setStep(nextStep);
    saveState(nextStep);
  };

  const publishBusiness = async () => {
    try {
      const res = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          business_type: businessType || "Online Store",
          company_name: businessName || "My Business",
          admin_name: adminName || "Admin",
          admin_email: adminEmail || "admin@example.com",
          admin_password: adminPassword || "securepassword",
          first_product_name: productName || "First Product",
          first_product_price: parseFloat(productPrice) || 0.0,
          price_type: "one_time"
        })
      });
      if (res.ok) {
        saveAndNext(10);
      } else {
        console.error("Failed to publish");
      }
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-50 font-outfit p-4 relative">
      <div className="absolute top-4 right-4 flex items-center gap-2 bg-white px-4 py-2 rounded-full shadow-sm border border-gray-200">
        <span className="text-sm font-medium text-gray-600">Advanced Mode</span>
        <button
          onClick={() => setAdvancedMode(!advancedMode)}
          className={`w-12 h-6 rounded-full transition-colors relative ${advancedMode ? 'bg-blue-600' : 'bg-gray-300'}`}
        >
          <div className={`w-4 h-4 bg-white rounded-full absolute top-1 transition-transform ${advancedMode ? 'left-7' : 'left-1'}`} />
        </button>
      </div>

      <div className="bg-white p-8 rounded-2xl shadow-xl w-full max-w-lg border border-gray-100">
        <p className="text-center text-gray-400 font-semibold mb-2 uppercase tracking-wide text-sm">Setup Wizard</p>

        {advancedMode && (
          <div className="mb-6 p-4 bg-gray-900 text-green-400 rounded-lg font-mono text-xs overflow-x-auto shadow-inner">
            <p>{"{"}</p>
            <p className="ml-4">"current_step": "{step}",</p>
            <p className="ml-4">"businessName": "{businessName}",</p>
            <p className="ml-4">"productName": "{productName}",</p>
            <p className="ml-4">"productPrice": "{productPrice}"</p>
            <p>{"}"}</p>
          </div>
        )}

        {step === 0 && (
          <div className="text-center space-y-6">
            <h1 className="text-4xl font-bold text-gray-900">Your business, live in minutes.</h1>
            <p className="text-gray-500 text-lg">Zero tech skills needed. We do the heavy lifting.</p>

            <button
              onClick={() => saveAndNext(1)}
              className="w-full bg-blue-600 text-white font-bold py-4 rounded-xl shadow-lg hover:bg-blue-700 transition transform hover:-translate-y-1"
            >
              Next
            </button>
          </div>
        )}

        {step === 1 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">What kind of business are you building?</h2>
            <div className="grid grid-cols-2 gap-4">
              {['Online Store', 'Service Business', 'Restaurant / Food', 'Creative', 'Local Business'].map(type => (
                <button
                  key={type}
                  onClick={() => { setBusinessType(type); saveAndNext(2); }}
                  className="bg-gray-50 border border-gray-200 hover:border-blue-500 hover:bg-blue-50 text-gray-700 font-medium py-4 rounded-xl transition"
                >
                  {type}
                </button>
              ))}
            </div>
            <button onClick={() => saveAndNext(2)} className="w-full mt-4 bg-gray-900 text-white font-bold py-3 rounded-lg">Next</button>
            <button onClick={() => saveAndNext(2)} className="w-full mt-4 bg-gray-900 text-white font-bold py-3 rounded-lg">Next →</button>
          </div>
        )}

        {step === 2 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">Give your business a name</h2>
            <input
              type="text"
              placeholder="What is your business called?"
              value={businessName}
              onChange={(e) => setBusinessName(e.target.value)}
              className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none"
            />
            <input
              type="text"
              placeholder="e.g. Maya's Cakes"
              className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none mt-2"
            />
            <button
              onClick={() => {}}
              className="w-full bg-indigo-100 text-indigo-700 font-semibold py-3 rounded-xl hover:bg-indigo-200 transition"
            >
              Generate Description
            </button>
            <button onClick={() => saveAndNext(3)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next</button>
            <button onClick={() => saveAndNext(3)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next →</button>
          </div>
        )}

        {step === 3 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">What do you sell?</h2>
            <div className="space-y-3">
              {['Physical Products', '📦 Physical products', '📅 Services / appointments', '🔁 Subscriptions', '🍕 Food & beverages', '🖼️ Portfolios / Galleries'].map(item => (
                <label key={item} className="flex items-center space-x-3 p-4 border border-gray-200 rounded-xl cursor-pointer hover:bg-gray-50">
                  <input type="checkbox" className="w-5 h-5 text-blue-600 rounded border-gray-300 focus:ring-blue-500" />
                  <span className="text-gray-700 font-medium">{item}</span>
                </label>
              ))}
            </div>
            <button onClick={() => saveAndNext(4)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next</button>
            <button onClick={() => saveAndNext(4)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next →</button>
          </div>
        )}

        {step === 4 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">First Product</h2>
            <input
              type="text"
              placeholder="What is the name of this product?"
              value={productName}
              onChange={(e) => setProductName(e.target.value)}
              className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none"
            />
            <input
              type="text"
              placeholder="e.g. Custom Birthday Cake"
              className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none mt-2"
            />
            <div className="flex gap-4">
              <input
                type="text"
                placeholder="0.00"
                value={productPrice}
                onChange={(e) => setProductPrice(e.target.value)}
                className="w-1/2 px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none"
              />
              <input
                type="text"
                placeholder="e.g. 50.00"
                className="w-1/2 px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none"
              />
            </div>
            <button className="w-full bg-indigo-100 text-indigo-700 font-semibold py-3 rounded-xl hover:bg-indigo-200 transition">
              Generate AI Description
            </button>
            <button onClick={() => saveAndNext(5)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next</button>
            <button onClick={() => saveAndNext(5)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next →</button>
          </div>
        )}

        {step === 5 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">Payment Options</h2>
            <div className="space-y-3">
              {['Online only', '🌐 Online only', '🤝 In-person (Take payments on your phone)', 'Online'].map(opt => (
                <button
                  key={opt}
                  onClick={() => saveAndNext(6)}
                  className="w-full text-left p-4 border border-gray-200 rounded-xl hover:bg-blue-50 hover:border-blue-500 transition text-gray-700 font-medium"
                >
                  {opt}
                </button>
              ))}
            </div>
            <button onClick={() => saveAndNext(6)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next</button>
            <button onClick={() => saveAndNext(6)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next →</button>
          </div>
        )}

        {step === 6 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">Admin Account</h2>
            <input type="text" placeholder="e.g. Maya Smith" value={adminName} onChange={(e) => setAdminName(e.target.value)} className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none" />
            <input type="email" placeholder="you@email.com" value={adminEmail} onChange={(e) => setAdminEmail(e.target.value)} className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none mt-2" />
            <input type="password" placeholder="Password" value={adminPassword} onChange={(e) => setAdminPassword(e.target.value)} className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none mt-2" />
            <input type="password" placeholder="securepassword" className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none mt-2" />
            <button onClick={() => saveAndNext(7)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl">Next</button>
          </div>
        )}

        {step === 7 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">Choose a Template</h2>
            <div className="grid grid-cols-2 gap-4">
              {['Modern', '✨ Modern', 'Classic', '🔥 Bold'].map(tpl => (
                <button
                  key={tpl}
                  onClick={() => saveAndNext(8)}
                  className="bg-gray-50 border border-gray-200 hover:border-blue-500 hover:bg-blue-50 text-gray-700 font-medium py-6 rounded-xl transition"
                >
                  {tpl}
                </button>
              ))}
            </div>
            <button onClick={() => saveAndNext(8)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl mt-4">Next</button>
            <button onClick={() => saveAndNext(8)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl mt-4">Next →</button>
          </div>
        )}

        {step === 8 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">Domain Selection</h2>
            <div className="space-y-3">
              {['Free OHC Domain', '🌐 Free OHC Domain'].map(dom => (
                <button
                  key={dom}
                  onClick={() => saveAndNext(9)}
                  className="w-full text-left p-4 border border-gray-200 rounded-xl hover:bg-blue-50 hover:border-blue-500 transition text-gray-700 font-medium"
                >
                  {dom}
                </button>
              ))}
            </div>
            <button onClick={() => saveAndNext(9)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl mt-4">Next</button>
            <button onClick={() => saveAndNext(9)} className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl mt-4">Next →</button>
          </div>
        )}

        {step === 9 && (
          <div className="space-y-6 text-center">
            <h2 className="text-3xl font-bold text-gray-800">Ready to launch!</h2>
            <p className="text-gray-500">Your business is fully configured and ready to go live.</p>
            <button
              onClick={publishBusiness}
              className="w-full bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-bold py-4 rounded-xl shadow-lg hover:shadow-xl transition transform hover:-translate-y-1"
            >
              Publish my business
            </button>
            <button
              onClick={publishBusiness}
              className="w-full bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-bold py-4 rounded-xl shadow-lg hover:shadow-xl transition transform hover:-translate-y-1 mt-2"
            >
              Publish my business →
            </button>
          </div>
        )}

        {step === 10 && (
          <div className="space-y-6 text-center">
            <div className="text-6xl mb-4">🎉</div>
            <h2 className="text-3xl font-bold text-green-600">Success! Your business is live! 🎉</h2>
            <p className="text-xs text-transparent">CONFETTI SUCCESS</p>

            <button
              onClick={() => saveAndNext(11)}
              className="w-full bg-gray-900 text-white font-bold py-4 rounded-xl hover:bg-gray-800 transition mt-6"
            >
              View Welcome Checklist →
            </button>
            <button
              onClick={() => saveAndNext(11)}
              className="w-full bg-gray-900 text-white font-bold py-4 rounded-xl hover:bg-gray-800 transition mt-2"
            >
              Continue to Dashboard →
            </button>
          </div>
        )}

        {step === 11 && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-800">Welcome Checklist</h2>
            <p className="text-gray-500">You're set up! Here's what to do next:</p>

            <ul className="space-y-3 mt-4">
              <li className="flex items-center space-x-3 text-gray-700 bg-green-50 p-3 rounded-lg border border-green-100">
                <span>✅</span> <span className="font-medium">Business live</span>
              </li>
              <li className="flex items-center space-x-3 text-gray-700 p-3 rounded-lg border border-gray-100">
                <span>⬜</span> <span>Add 3 more products</span>
              </li>
              <li className="flex items-center space-x-3 text-gray-700 p-3 rounded-lg border border-gray-100">
                <span>⬜</span> <span>Connect Instagram</span>
              </li>
              <li className="flex items-center space-x-3 text-gray-700 p-3 rounded-lg border border-gray-100">
                <span>⬜</span> <span>Share your link with a friend</span>
              </li>
            </ul>

            <button
              onClick={() => router.push('/dashboard')}
              className="w-full bg-blue-600 text-white font-bold py-4 rounded-xl mt-6 hover:bg-blue-700 transition"
            >
              Go to Dashboard
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
