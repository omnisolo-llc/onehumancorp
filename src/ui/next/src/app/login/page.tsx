'use client';
import { useRouter } from 'next/navigation';
import React, { useState } from 'react';

export default function Login() {
  const router = useRouter();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    router.push('/dashboard');
  };

  return (
    <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden justify-center items-center mac-glass-container">
      <div className="w-[85%] mb-8 text-center">
        <h1 className="text-[28px] font-outfit font-bold text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">Login</h1>
        <p className="text-[#6F7A8B] text-sm mt-2 font-medium">Access your OHC Workspace</p>
      </div>

      <form className="w-[85%] flex flex-col gap-4" onSubmit={handleLogin}>
        <div>
          <label className="block text-xs font-bold text-[#6F7A8B] uppercase tracking-wider mb-2">Email</label>
          <input
            className="w-full p-4 border border-white/50 dark:border-white/10 mac-glass-container rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all shadow-inner"
            type="text"
            placeholder="Email or Username"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />
        </div>

        <div>
          <label className="block text-xs font-bold text-[#6F7A8B] uppercase tracking-wider mb-2">Password</label>
          <input
            className="w-full p-4 border border-white/50 dark:border-white/10 mac-glass-container rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all shadow-inner"
            type="password"
            placeholder="••••••••"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
        </div>

        <button
          className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold text-sm min-h-[44px] hover:bg-[#0052cc] active:scale-95 transition-all shadow-md mt-4"
          type="submit"
        >
          Login
        </button>
      </form>
    </div>
  );
}
