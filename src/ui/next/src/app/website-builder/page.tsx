"use client";

import { useState, useEffect } from "react";
import { SmartBlock } from "../builder/components";
import { Tooltip, useWalkthrough } from "../../components/help";

export default function WebsiteBuilderPage() {
  const [step, setStep] = useState(0);
  const [businessType, setBusinessType] = useState("");
  const [companyName, setCompanyName] = useState("");
  const [sellingCategories, setSellingCategories] = useState<string[]>([]);
  const [productName, setProductName] = useState("");
  const [productPrice, setProductPrice] = useState("");
  const [paymentPref, setPaymentPref] = useState("");
  const [adminName, setAdminName] = useState("");
  const [adminEmail, setAdminEmail] = useState("");
  const [adminPassword, setAdminPassword] = useState("");
  const [websiteTemplate, setWebsiteTemplate] = useState("");
  const [domainChoice, setDomainChoice] = useState("");

  const [bio, setBio] = useState("");
  const [blocks, setBlocks] = useState<any[]>([]);
  const [status, setStatus] = useState<"idle" | "generating" | "draft" | "live">("idle");
  const [liveUrl, setLiveUrl] = useState("");
  const [tenantId, setTenantId] = useState("storefront");
  const { startWalkthrough } = useWalkthrough();

  useEffect(() => {
    const savedTenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "storefront";
    setTenantId(savedTenantId);

    const savedBio = localStorage.getItem("ohc_builder_bio");
    if (savedBio) setBio(savedBio);

    const savedStatus = localStorage.getItem("ohc_builder_status") as "idle" | "generating" | "draft" | "live";
    if (savedStatus) setStatus(savedStatus);

    const savedBlocks = localStorage.getItem("ohc_builder_blocks");
    if (savedBlocks) {
      try {
        setBlocks(JSON.parse(savedBlocks));
      } catch (e) {
        console.error("Failed to parse saved blocks", e);
      }
    }

    const savedLiveUrl = localStorage.getItem("ohc_builder_liveUrl");
    if (savedLiveUrl) setLiveUrl(savedLiveUrl);

    fetch('/api/onboarding/state', {
      headers: {
        'x-tenant-id': savedTenantId,
        'x-user-id': localStorage.getItem('user_id') || 'test-user'
      }
    })
    .then(res => res.json())
    .then(data => {
      if (data && Object.keys(data).length > 0) {
        if (data.step !== undefined) setStep(data.step);
        if (data.businessType) setBusinessType(data.businessType);
        if (data.companyName) setCompanyName(data.companyName);
        if (data.sellingCategories) setSellingCategories(data.sellingCategories);
        if (data.productName) setProductName(data.productName);
        if (data.productPrice) setProductPrice(data.productPrice);
        if (data.paymentPref) setPaymentPref(data.paymentPref);
        if (data.adminName) setAdminName(data.adminName);
        if (data.adminEmail) setAdminEmail(data.adminEmail);
        // Do not load password from state
        if (data.websiteTemplate) setWebsiteTemplate(data.websiteTemplate);
        if (data.domainChoice) setDomainChoice(data.domainChoice);
        if (data.bio) setBio(data.bio);
      }
    })
    .catch(err => console.error('Failed to load builder state', err));
  }, []);

  const updateBio = (newBio: string) => {
    setBio(newBio);
    localStorage.setItem("ohc_builder_bio", newBio);
  };

  const updateStatus = (newStatus: "idle" | "generating" | "draft" | "live") => {
    setStatus(newStatus);
    localStorage.setItem("ohc_builder_status", newStatus);
  };

  useEffect(() => {
    if (step === 0 && !businessType && !companyName) return; // don't sync initial empty state

    const timer = setTimeout(() => {
      const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
      const userId = localStorage.getItem('user_id') || 'test-user';
      fetch('/api/onboarding/state', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'x-tenant-id': tenantId, 'x-user-id': userId },
        body: JSON.stringify({
          step, businessType, companyName, sellingCategories, productName, productPrice,
          paymentPref, adminName, adminEmail, websiteTemplate, domainChoice, bio
        })
      });
    }, 500);
    return () => clearTimeout(timer);
  }, [step, businessType, companyName, sellingCategories, productName, productPrice, paymentPref, adminName, adminEmail, websiteTemplate, domainChoice, bio]);


  const toggleCategory = (cat: string) => {
    setSellingCategories(prev =>
      prev.includes(cat) ? prev.filter(c => c !== cat) : [...prev, cat]
    );
  };

  const handleGenerate = async () => {
    setStatus("generating");

    try {
      const response = await fetch('/api/v1/builder/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: bio })
      });

      const data = await response.json();
      const blocks = data.pages[0].blocks.map((b: any) => ({
        type: b.block_type === 'HeroBlock' ? 'Hero' :
              b.block_type === 'ProductGridBlock' ? 'Catalog' :
              b.block_type === 'ServiceBookingBlock' ? 'Booking' :
              b.block_type === 'TestimonialBlock' ? 'Testimonials' : b.block_type,
        props: b.content
      }));
      setBlocks(blocks);
      localStorage.setItem("ohc_builder_blocks", JSON.stringify(blocks));
      updateStatus("draft");
    } catch (error) {
      console.error("Failed to generate storefront", error);
      updateStatus("idle");
    }
  };

  const handleLaunch = async () => {
    try {
      const payload = {
        business_type: businessType || "Retail",
        company_name: companyName,
        selling_categories: sellingCategories,
        first_product_name: productName,
        first_product_price: productPrice,
        payment_pref: paymentPref,
        admin_name: adminName,
        admin_email: adminEmail,
        admin_password: adminPassword, // include password ONLY here
        website_template: websiteTemplate,
        domain_choice: domainChoice
      };

      const response = await fetch('/api/onboarding/start', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
      });
      if (response.ok) {
        setStep(10);
      } else {
        console.error('Failed to publish');
      }
    } catch (error) {
      console.error('Error publishing:', error);
    }
  };

  const handlePublishDraft = async () => {
    try {
      const draftBlocks = blocks.map((b, i) => ({
        block_type: b.type === 'Hero' ? 'HeroBlock' :
                    b.type === 'Catalog' ? 'ProductGridBlock' :
                    b.type === 'Booking' ? 'ServiceBookingBlock' :
                    b.type === 'Testimonials' ? 'TestimonialBlock' : b.type,
        content: b.props,
        sort_order: i
      }));

      const payload = {
          domain: null,
          draft: {
              domain: null,
              pages: [{
                  path: '/',
                  title: 'Home',
                  blocks: draftBlocks,
                  seo_metadata: {
                    "@context": "https://schema.org",
                    "@type": "LocalBusiness",
                    "name": bio
                  }
              }]
          }
      };

      const response = await fetch('/api/v1/builder/publish_draft', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
      });
      if (response.ok) {
        const data = await response.json();
        updateStatus("live");
        const url = `https://${data.domain || 'myshop'}.ohc.store`;
        setLiveUrl(url);
        localStorage.setItem("ohc_builder_liveUrl", url);
      } else {
        console.error('Failed to publish draft');
      }
    } catch (error) {
      console.error('Error publishing draft:', error);
    }
  };


  if (status === "generating") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] shadow-2xl flex flex-col relative justify-center items-center overflow-hidden bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[16px]">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mb-4"></div>
            <p className="text-gray-500 dark:text-[#a1a1a6] font-medium">Agents are building your store...</p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] shadow-2xl flex flex-col relative text-center p-8 justify-center overflow-hidden bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[16px]">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 shadow-sm">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">You're Live!</h1>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">Your automated storefront is successfully published.</p>

          <div className="w-full bg-gray-50 p-3 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 dark:text-[#a1a1a6] truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline shrink-0">Copy</button>
          </div>

          <button
            className="w-full bg-gray-100 text-gray-800 dark:text-[#f5f5f7] font-bold p-4 active:scale-[0.98] transition-all hover:bg-gray-200"
            style={{ borderRadius: '8px' }}
            onClick={() => updateStatus("idle")}
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }

  if (status === "draft") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] shadow-2xl flex flex-col relative overflow-hidden bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[16px]">
          <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
            <span>Preview Mode</span>
            <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
          </div>

          <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
            {blocks.map((b, i) => (
              <SmartBlock key={i} {...b} />
            ))}
            <SmartBlock type="PoweredBy" props={{ tenantId }} />
          </div>

          <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-md border-t border-gray-200 z-50" style={{ borderRadius: '0 0 16px 16px' }}>
            <Tooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
              <button
                id="launch-btn"
                className="w-full bg-blue-600 text-white p-4 font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
                style={{ borderRadius: '8px' }}
                onClick={handlePublishDraft}
              >
                <span>1-Tap Launch</span>
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
              </button>
            </Tooltip>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div id="setup-screen" className="w-[375px] h-[812px] shadow-2xl flex flex-col relative overflow-hidden bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[16px]">

        <div className="flex-1 overflow-y-auto p-6 flex flex-col justify-center">

          {step === 0 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-8">Your business, live in minutes.</h1>
              <button
                className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] mb-4 hover:bg-blue-700 active:scale-[0.98] transition-all"
                onClick={() => setStep(1)}
              >
                Start My Business
              </button>
              <button
                className="w-full bg-gray-200 text-gray-800 p-4 font-bold rounded-[8px] hover:bg-gray-300 active:scale-[0.98] transition-all"
                onClick={() => setStep(100)}
              >
                Instant Build
              </button>
            </div>
          )}

          {step === 100 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-4">Describe your business in a sentence</h1>
              <input
                className="w-full border border-gray-200 bg-white/70 p-4 mb-4 rounded-[8px] focus:ring-2 focus:ring-[#0071E3] outline-none"
                placeholder="I run a local bakery called Maya Cakes."
                value={bio}
                onChange={(e) => updateBio(e.target.value)}
              />
              <button
                className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]"
                onClick={handleGenerate}
              >
                Generate Storefront
              </button>
            </div>
          )}

          {step === 1 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">What kind of business are you building?</h1>
              <div className="grid grid-cols-2 gap-4 mb-6">
                {["Creative", "Retail", "Services", "Food", "Online Store", "Restaurant"].map(type => (
                  <button
                    key={type}
                    className={`p-4 rounded-[8px] border ${businessType === type ? 'bg-blue-50 border-blue-500 text-blue-700' : 'bg-white border-gray-200'} font-medium`}
                    onClick={() => setBusinessType(type)}
                  >
                    {type}
                  </button>
                ))}
              </div>
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(2)}>Next</button>
            </div>
          )}

          {step === 2 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">Give your business a name</h1>
              <input
                className="w-full border border-gray-200 bg-white/70 p-4 mb-6 rounded-[8px] focus:ring-2 focus:ring-[#0071E3] outline-none"
                placeholder="What is your business called?"
                value={companyName}
                onChange={(e) => setCompanyName(e.target.value)}
              />
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(3)}>Next</button>
            </div>
          )}

          {step === 3 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">What do you sell?</h1>
              <div className="flex flex-col gap-4 mb-6 text-left">
                {["Services", "Physical Products", "Digital Products"].map(cat => (
                  <label key={cat} className="flex items-center gap-3 p-4 bg-white border border-gray-200 rounded-[8px]">
                    <input
                      type="checkbox"
                      checked={sellingCategories.includes(cat)}
                      onChange={() => toggleCategory(cat)}
                      className="w-5 h-5"
                    />
                    <span className="font-medium text-gray-800">{cat}</span>
                  </label>
                ))}
              </div>
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(4)}>Next</button>
            </div>
          )}

          {step === 4 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">Let's add your first product</h1>
              <input
                className="w-full border border-gray-200 bg-white/70 p-4 mb-4 rounded-[8px] focus:ring-2 focus:ring-[#0071E3] outline-none"
                placeholder="What is the name of this product?"
                value={productName}
                onChange={(e) => setProductName(e.target.value)}
              />
              <input
                className="w-full border border-gray-200 bg-white/70 p-4 mb-6 rounded-[8px] focus:ring-2 focus:ring-[#0071E3] outline-none"
                placeholder="0.00"
                value={productPrice}
                onChange={(e) => setProductPrice(e.target.value)}
              />
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(5)}>Next</button>
            </div>
          )}

          {step === 5 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">How do you want to receive payments?</h1>
              <div className="flex flex-col gap-4 mb-6">
                {["Online", "Both Online", "In Person"].map(pref => (
                  <button
                    key={pref}
                    className={`p-4 rounded-[8px] border ${paymentPref === pref ? 'bg-blue-50 border-blue-500 text-blue-700' : 'bg-white border-gray-200'} font-medium`}
                    onClick={() => setPaymentPref(pref)}
                  >
                    {pref}
                  </button>
                ))}
              </div>
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(6)}>Next</button>
            </div>
          )}

          {step === 6 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">Create your admin account</h1>
              <input
                className="w-full border border-gray-200 bg-white/70 p-4 mb-4 rounded-[8px] focus:ring-2 focus:ring-[#0071E3] outline-none"
                placeholder="e.g. Maya Smith"
                value={adminName}
                onChange={(e) => setAdminName(e.target.value)}
              />
              <input
                className="w-full border border-gray-200 bg-white/70 p-4 mb-4 rounded-[8px] focus:ring-2 focus:ring-[#0071E3] outline-none"
                placeholder="you@email.com"
                value={adminEmail}
                onChange={(e) => setAdminEmail(e.target.value)}
              />
              <input
                type="password"
                className="w-full border border-gray-200 bg-white/70 p-4 mb-6 rounded-[8px] focus:ring-2 focus:ring-[#0071E3] outline-none"
                placeholder="Password"
                value={adminPassword}
                onChange={(e) => setAdminPassword(e.target.value)}
              />
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(7)}>Next</button>
            </div>
          )}

          {step === 7 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">Choose a starting template</h1>
              <div className="flex flex-col gap-4 mb-6">
                {["Modern", "Classic", "Bold"].map(tpl => (
                  <button
                    key={tpl}
                    className={`p-4 rounded-[8px] border ${websiteTemplate === tpl ? 'bg-blue-50 border-blue-500 text-blue-700' : 'bg-white border-gray-200'} font-medium`}
                    onClick={() => setWebsiteTemplate(tpl)}
                  >
                    {tpl}
                  </button>
                ))}
              </div>
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(8)}>Next</button>
            </div>
          )}

          {step === 8 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">Choose your domain</h1>
              <div className="flex flex-col gap-4 mb-6">
                {["Free OHC Domain", "Connect Custom Domain", "Free OHC Subdomain"].map(dom => (
                  <button
                    key={dom}
                    className={`p-4 rounded-[8px] border ${domainChoice === dom ? 'bg-blue-50 border-blue-500 text-blue-700' : 'bg-white border-gray-200'} font-medium`}
                    onClick={() => setDomainChoice(dom)}
                  >
                    {dom}
                  </button>
                ))}
              </div>
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(9)}>Next</button>
            </div>
          )}

          {step === 9 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-6">Ready to launch!</h1>
              <button className="w-full bg-green-500 text-white p-4 font-bold rounded-[8px]" onClick={handleLaunch}>Publish my business</button>
            </div>
          )}

          {step === 10 && (
            <div className="animate-fade-in text-center">
              <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-4">CONFETTI SUCCESS</h1>
              <p className="text-gray-500 dark:text-[#a1a1a6] font-medium mb-6">Your business is now live!</p>
              <button className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px]" onClick={() => setStep(11)}>View Welcome Checklist</button>
            </div>
          )}

          {step === 11 && (
            <div className="animate-fade-in text-center">
              <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-4">You're set up! Here's what to do next:</h2>
              <ul className="text-left text-gray-600 mb-6 space-y-2">
                <li>✅ Verify email</li>
                <li>✅ Connect bank account</li>
                <li>✅ Share store link</li>
              </ul>
              <button className="w-full bg-gray-100 text-gray-800 p-4 font-bold rounded-[8px]" onClick={() => window.location.href='/dashboard'}>Go to Dashboard</button>
            </div>
          )}

        </div>
      </div>
    </div>
  );
}
