'use client';
import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function Onboarding() {
  const router = useRouter();
  const [description, setDescription] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const [storeProfile, setStoreProfile] = useState<any>(null);

  useEffect(() => {
    const hasOnboarded = localStorage.getItem('has_onboarded');
    if (hasOnboarded) {
      router.push('/dashboard');
    }
  }, [router]);

  const handleGenerateStore = async () => {
    if (!description || description.length < 5) {
      setError('Please provide a valid description (at least 5 characters).');
      return;
    }
    setIsLoading(true);
    setError('');

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const res = await fetch('/api/onboarding/zero_touch', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description })
      });

      if (!res.ok) throw new Error('Failed to generate store profile');
      const data = await res.json();
      setStoreProfile(data);
    } catch (err: any) {
      setError(err.message || 'An error occurred while generating the store profile.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleLaunchStore = async () => {
    setIsLoading(true);
    setError('');

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const res = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({
          business_type: storeProfile.business_type,
          company_name: storeProfile.business_name,
          company_description: description,
          selling_categories: storeProfile.selling_categories,
          website_template: storeProfile.theme,
          domain_choice: 'subdomain',
          first_product_name: storeProfile.sample_products?.[0]?.name || '',
          first_product_price: storeProfile.sample_products?.[0]?.price || '0.00',
          price_type: storeProfile.price_type || 'fixed',
          location: storeProfile.default_location || 'Remote',
          payment_pref: 'online',
          admin_email: 'test@example.com',
          admin_name: 'Test User',
          admin_password: 'password123',
        })
      });

      if (!res.ok) throw new Error('Failed to launch store');
      localStorage.setItem('has_onboarded', 'true');
      router.push('/dashboard');
    } catch (err: any) {
      setError(err.message || 'An error occurred while launching the store.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-[100dvh] bg-white dark:bg-black font-inter selection:bg-[#0066FF] selection:text-white flex flex-col items-center p-4">
      <div className="w-full max-w-[375px] flex-1 flex flex-col mt-10">
        {!isLoading && !storeProfile && (
          <div className="animate-fade-in flex flex-col flex-1">
            <h1 className="text-3xl font-bold font-outfit tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Build Your Business</h1>
            <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">Describe your business, and our AI agents will build your entire store in seconds.</p>

            <div className="mb-6 relative">
              <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Business Description</label>
              <textarea
                placeholder="e.g., I sell custom vegan cakes in Seattle."
                className="w-full min-h-[150px] p-4 rounded-[12px] border border-gray-200 dark:border-white/10 bg-white/50 dark:bg-black/30 backdrop-blur-xl focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/10 transition-all outline-none resize-none text-[#1D1D1F] dark:text-white placeholder:text-gray-400"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
              />
              <div className="absolute right-3 bottom-3 p-2 bg-gray-100 dark:bg-white/10 rounded-full cursor-pointer hover:bg-gray-200 dark:hover:bg-white/20 transition-colors">
                 <svg className="w-5 h-5 text-gray-500 dark:text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
              </div>
            </div>

            {error && <div className="text-red-500 text-sm mb-4 font-medium px-2">{error}</div>}

            <div className="mt-auto pt-6">
              <button
                onClick={handleGenerateStore}
                className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[12px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 disabled:opacity-50"
              >
                Generate My Business
              </button>
            </div>
          </div>
        )}

        {isLoading && (
          <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in mac-glass-container backdrop-blur-2xl rounded-2xl p-8 border border-white/20">
             <div className="w-24 h-24 relative mb-8">
               <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
               <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
             </div>
             <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Agents at work...</h2>
             <div className="space-y-2">
               <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Designing your storefront</p>
               <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Configuring inventory & shipping</p>
               <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Setting up payment profile</p>
             </div>
          </div>
        )}

        {!isLoading && storeProfile && (
          <div className="animate-fade-in flex flex-col flex-1">
             <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-6">Review Details</h2>

             <div className="space-y-4 mb-6 flex-1">
                <div className="p-4 mac-glass-container backdrop-blur-md rounded-[12px] border border-white/50 dark:border-white/10">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-1">Business Name</p>
                   <p className="text-[#1D1D1F] dark:text-white font-semibold">{storeProfile.business_name}</p>
                </div>
                <div className="p-4 mac-glass-container backdrop-blur-md rounded-[12px] border border-white/50 dark:border-white/10">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-1">Business Type</p>
                   <p className="text-[#1D1D1F] dark:text-white font-semibold">{storeProfile.business_type}</p>
                </div>
                <div className="p-4 mac-glass-container backdrop-blur-md rounded-[12px] border border-white/50 dark:border-white/10">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-1">Theme</p>
                   <p className="text-[#1D1D1F] dark:text-white font-semibold">{storeProfile.theme}</p>
                </div>
                <div className="p-4 mac-glass-container backdrop-blur-md rounded-[12px] border border-white/50 dark:border-white/10">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Sample Products</p>
                   <ul className="space-y-2">
                     {storeProfile.sample_products?.map((p: any, i: number) => (
                       <li key={i} className="flex justify-between items-center text-sm border-b border-gray-100 dark:border-white/5 pb-2 last:border-0 last:pb-0">
                         <span className="text-[#1D1D1F] dark:text-white">{p.name}</span>
                         <span className="font-medium text-gray-500 dark:text-[#A1A1A6]">${p.price}</span>
                       </li>
                     ))}
                   </ul>
                </div>
             </div>

             {error && <div className="text-red-500 text-sm mb-4 font-medium px-2">{error}</div>}

             <div className="mt-auto pt-4 flex gap-3">
               <button
                 onClick={() => setStoreProfile(null)}
                 className="flex-1 mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] border border-white/50 dark:border-white/10 min-h-[54px] p-4 rounded-[12px] font-bold shadow-sm active:scale-[0.98] transition-all"
               >
                 Back
               </button>
               <button
                 onClick={handleLaunchStore}
                 className="flex-[2] bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[12px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all"
               >
                 Approve & Go Live
               </button>
             </div>
          </div>
        )}
      </div>
    </div>
  );
}
