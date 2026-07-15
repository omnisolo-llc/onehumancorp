"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ViralQuizGenerator() {
  const router = useRouter();
  const [tenant, setTenant] = useState('default');

  // Builder state
  const [quizTitle, setQuizTitle] = useState('What type of founder are you?');
  const [quizDescription, setQuizDescription] = useState('Take this 30-second quiz to find out your true entrepreneurial style!');
  const [questionText, setQuestionText] = useState('How do you handle a sudden crisis?');
  const [option1, setOption1] = useState('Take immediate charge');
  const [option2, setOption2] = useState('Analyze the data first');
  const [option3, setOption3] = useState('Collaborate with the team');
  const [option4, setOption4] = useState('Delegate and trust');
  const [resultText, setResultText] = useState('You are the Visionary! 🚀 Share to unlock your detailed report.');

  // Quiz interactive state
  const [quizState, setQuizState] = useState<'start' | 'question' | 'result'>('start');
  const [selectedOption, setSelectedOption] = useState<number | null>(null);

  // Sharing state
  const [copied, setCopied] = useState(false);
  const [generatedLink, setGeneratedLink] = useState('');

  useEffect(() => {
    let currentTenant = 'default';
    if (typeof window !== 'undefined') {
      currentTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'default';
      setTenant(currentTenant);
    }

    // Auto-generate a preview link
    const baseUrl = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
    const params = new URLSearchParams({
      tenant: currentTenant,
      title: quizTitle,
      source: 'viral_quiz_generator'
    });
    setGeneratedLink(`${baseUrl}/q?${params.toString()}`);
  }, [quizTitle]);

  const handleCopy = () => {
    navigator.clipboard.writeText(generatedLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleStart = () => {
    setQuizState('question');
    setSelectedOption(null);
  };

  const handleOptionSelect = (index: number) => {
    setSelectedOption(index);
    setTimeout(() => {
      setQuizState('result');
    }, 600);
  };

  const handleReset = () => {
    setQuizState('start');
    setSelectedOption(null);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Viral Quiz Generator 🧠</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors rounded-lg"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-7xl mx-auto w-full flex flex-col lg:flex-row gap-8">

        {/* Left Side: Builder Controls */}
        <div className="w-full lg:w-1/3 flex flex-col gap-6">
          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-white/60 backdrop-blur-[40px] saturate-[200%] border border-white/40 rounded-2xl">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Quiz Settings</h2>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Quiz Title</label>
              <input
                type="text"
                value={quizTitle}
                onChange={(e) => setQuizTitle(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                placeholder="e.g. What type of founder are you?"
              />
            </div>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Description</label>
              <textarea
                value={quizDescription}
                onChange={(e) => setQuizDescription(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[80px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all resize-y"
                placeholder="Hook them in..."
              />
            </div>

            <div className="mb-4 pt-4 border-t border-gray-200">
              <label className="block text-sm font-medium text-gray-700 mb-2">Sample Question</label>
              <input
                type="text"
                value={questionText}
                onChange={(e) => setQuestionText(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all mb-3"
              />

              <div className="space-y-2 pl-2 border-l-2 border-[#0066FF]/20">
                  <input
                    type="text"
                    value={option1}
                    onChange={(e) => setOption1(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm text-sm min-h-[40px] focus:outline-none focus:ring-1 focus:ring-[#0066FF] transition-all"
                    placeholder="Option 1"
                  />
                  <input
                    type="text"
                    value={option2}
                    onChange={(e) => setOption2(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm text-sm min-h-[40px] focus:outline-none focus:ring-1 focus:ring-[#0066FF] transition-all"
                    placeholder="Option 2"
                  />
                  <input
                    type="text"
                    value={option3}
                    onChange={(e) => setOption3(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm text-sm min-h-[40px] focus:outline-none focus:ring-1 focus:ring-[#0066FF] transition-all"
                    placeholder="Option 3"
                  />
                  <input
                    type="text"
                    value={option4}
                    onChange={(e) => setOption4(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm text-sm min-h-[40px] focus:outline-none focus:ring-1 focus:ring-[#0066FF] transition-all"
                    placeholder="Option 4"
                  />
              </div>
            </div>

            <div className="mb-4 pt-4 border-t border-gray-200">
              <label className="block text-sm font-medium text-gray-700 mb-2">Result Teaser (Share to Unlock)</label>
              <textarea
                value={resultText}
                onChange={(e) => setResultText(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[80px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all resize-y"
              />
            </div>
          </div>

          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-indigo-50/70 backdrop-blur-[40px] border border-indigo-100/50 rounded-2xl">
            <h3 className="font-bold text-indigo-900 mb-2 flex items-center gap-2">
              <span className="text-xl">🚀</span> Share Your Quiz
            </h3>
            <p className="text-sm text-indigo-800 mb-4">
              Embed this interactive quiz on your site or share the link directly to capture leads!
            </p>

            <div className="flex items-center gap-2 bg-white min-h-[44px] min-w-[44px] border border-indigo-200 p-1 mb-4 rounded-lg overflow-hidden">
              <div className="px-2 py-1 text-xs text-gray-500 truncate flex-1 font-mono" data-testid="generated-link">
                {generatedLink}
              </div>
            </div>

            <button
              onClick={handleCopy}
              className="w-full py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors rounded-lg flex items-center justify-center gap-2"
            >
              {copied ? '✓ Copied!' : '📋 Copy Link'}
            </button>
          </div>
        </div>

        {/* Right Side: Interactive Preview */}
        <div className="w-full lg:w-2/3 flex flex-col">
          <div className="flex-1 shadow-[0_20px_40px_rgb(0,0,0,0.15)] overflow-hidden flex flex-col bg-white/40 backdrop-blur-[40px] saturate-[200%] border border-white/50 rounded-2xl relative min-h-[600px]">
            {/* Browser Header Mock */}
            <div className="bg-gray-100 py-3 px-4 flex items-center gap-2 border-b border-gray-200">
              <div className="flex gap-1.5">
                <div className="w-3 h-3 rounded-full bg-red-400"></div>
                <div className="w-3 h-3 rounded-full bg-amber-400"></div>
                <div className="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <div className="mx-auto bg-white text-xs text-gray-500 px-4 py-1 rounded-full w-1/2 text-center truncate border border-gray-200 shadow-sm">
                Live Preview
              </div>
            </div>

            <div className="flex-1 flex flex-col items-center justify-center p-8 bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 overflow-y-auto">

              <div className="w-full max-w-md ohc-growth-card glass-card shadow-2xl rounded-3xl p-8 relative overflow-hidden bg-white/80 backdrop-blur-xl border border-white flex flex-col">

                  {quizState === 'start' && (
                    <div className="flex flex-col items-center text-center animate-in fade-in zoom-in duration-500">
                        <div className="w-20 h-20 bg-indigo-100 text-indigo-500 rounded-full flex items-center justify-center text-4xl mb-6 shadow-inner">
                            🎯
                        </div>
                        <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-4">{quizTitle || 'Quiz Title'}</h2>
                        <p className="text-gray-600 mb-8 font-medium leading-relaxed">
                            {quizDescription || 'Description goes here...'}
                        </p>
                        <button
                            onClick={handleStart}
                            className="w-full py-4 bg-[#1D1D1F] hover:bg-black text-white font-bold rounded-xl text-lg min-h-[56px] transition-all shadow-lg hover:shadow-xl transform hover:-translate-y-1"
                        >
                            Start Quiz
                        </button>
                    </div>
                  )}

                  {quizState === 'question' && (
                    <div className="flex flex-col animate-in fade-in slide-in-from-right-8 duration-300 w-full">
                        <div className="w-full bg-gray-200 rounded-full h-2 mb-8">
                            <div className="bg-indigo-600 h-2 rounded-full w-1/3"></div>
                        </div>
                        <span className="text-sm font-bold text-indigo-600 uppercase tracking-wider mb-2">Question 1 of 3</span>
                        <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6 leading-tight">{questionText || 'Question text...'}</h2>

                        <div className="space-y-3 w-full">
                            {[option1, option2, option3, option4].filter(Boolean).map((opt, i) => (
                                <button
                                    key={i}
                                    onClick={() => handleOptionSelect(i)}
                                    className={`w-full p-4 rounded-xl text-left font-medium min-h-[56px] transition-all border-2 flex items-center justify-between group
                                        ${selectedOption === i
                                            ? 'border-indigo-600 bg-indigo-50 text-indigo-900 shadow-md scale-[1.02]'
                                            : 'border-gray-200 hover:border-indigo-300 hover:bg-indigo-50/50 bg-white'}`}
                                >
                                    <span>{opt}</span>
                                    <div className={`w-5 h-5 rounded-full border-2 flex items-center justify-center transition-colors
                                        ${selectedOption === i ? 'border-indigo-600' : 'border-gray-300 group-hover:border-indigo-400'}`}>
                                        {selectedOption === i && <div className="w-2.5 h-2.5 bg-indigo-600 rounded-full" />}
                                    </div>
                                </button>
                            ))}
                        </div>
                    </div>
                  )}

                  {quizState === 'result' && (
                    <div className="flex flex-col items-center text-center animate-in fade-in zoom-in duration-500 w-full">
                        <div className="w-24 h-24 bg-gradient-to-br from-pink-400 to-purple-600 text-white rounded-full flex items-center justify-center text-5xl mb-6 shadow-xl relative animate-bounce-slight">
                            ✨
                        </div>
                        <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-2">Your Results Are Ready!</h2>
                        <p className="text-gray-800 font-medium mb-6 text-lg p-4 bg-amber-50 rounded-xl border border-amber-200 w-full shadow-inner">
                            {resultText || 'Result text...'}
                        </p>

                        <div className="w-full bg-gray-50 border-2 border-dashed border-gray-300 rounded-xl p-6 mb-6 relative overflow-hidden">
                            <div className="absolute inset-0 bg-white/40 backdrop-blur-[2px] z-10 flex flex-col items-center justify-center">
                                <span className="bg-white px-4 py-2 rounded-full font-bold text-gray-900 shadow-md text-sm border border-gray-200 flex items-center gap-2">
                                    🔒 Share to View Full Report
                                </span>
                            </div>
                            <div className="blur-sm opacity-50 space-y-3">
                                <div className="h-4 bg-gray-300 rounded w-3/4"></div>
                                <div className="h-4 bg-gray-300 rounded w-full"></div>
                                <div className="h-4 bg-gray-300 rounded w-5/6"></div>
                                <div className="h-4 bg-gray-300 rounded w-1/2"></div>
                            </div>
                        </div>

                        <div className="w-full space-y-3 z-20">
                            <button className="w-full py-3.5 bg-black hover:bg-gray-800 text-white font-bold rounded-xl min-h-[52px] transition-all flex items-center justify-center gap-3 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5">
                                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"></path></svg>
                                Share on X to Unlock
                            </button>
                            <button className="w-full py-3.5 bg-[#0A66C2] hover:bg-[#004182] text-white font-bold rounded-xl min-h-[52px] transition-all flex items-center justify-center gap-3 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5">
                                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/></svg>
                                Share on LinkedIn
                            </button>

                            <button onClick={handleReset} className="w-full py-3 bg-white hover:bg-gray-50 text-gray-600 font-bold rounded-xl min-h-[52px] transition-all border border-gray-200 mt-4 text-sm">
                                Retake Quiz
                            </button>
                        </div>
                    </div>
                  )}

                  <div className="mt-8 pt-6 border-t w-full text-center border-gray-100 flex justify-center">
                    <PoweredByOHC tenantId={tenant} />
                  </div>
              </div>

            </div>
          </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .animate-bounce-slight { animation: bounce-slight 2s infinite ease-in-out; }
        @keyframes bounce-slight {
            0%, 100% { transform: translateY(-5%); }
            50% { transform: translateY(0); }
        }
      `}} />
    </div>
  );
}
