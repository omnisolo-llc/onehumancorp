"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const router = useRouter();

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    localStorage.setItem("user_id", "test-user");
    router.push("/dashboard");
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      <div className="w-full max-w-[375px] mx-auto mac-glass-container rounded-[16px] shadow-lg overflow-hidden flex flex-col p-6">
        <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-6">Login</h1>
        <form onSubmit={handleLogin} className="space-y-4">
          <div>
            <input
              type="text"
              placeholder="Email or Username"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full p-4 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none bg-white/60 dark:bg-black/30 backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
            />
          </div>
          <div>
            <input
              type="password"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full p-4 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none bg-white/60 dark:bg-black/30 backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
            />
          </div>
          <button
            type="submit"
            className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
          >
            Login
          </button>
        </form>
      </div>
    </div>
  );
}
