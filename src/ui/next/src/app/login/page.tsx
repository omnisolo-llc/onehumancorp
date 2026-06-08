'use client';
import { useRouter } from 'next/navigation';

export default function Login() {
  const router = useRouter();
  const goDashboard = () => router.push('/dashboard');

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 p-4 font-outfit">
      <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col p-8 sm:p-12 relative rounded-[24px] glassmorphism border border-white/20 shadow-2xl">
        <h1 className="text-3xl font-bold text-center text-[#1D1D1F] dark:text-[#F5F5F7] mb-8">Login</h1>

        <div className="flex flex-col gap-4 mb-8">
          <input
            type="text"
            placeholder="Email or Username"
            className="w-full p-4 rounded-[8px] focus:border-[#0066FF] outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
          />
          <input
            type="password"
            placeholder="Password"
            className="w-full p-4 rounded-[8px] focus:border-[#0066FF] outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
          />
          <button
            onClick={goDashboard}
            className="w-full bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,0,0,0.39)] hover:bg-black dark:hover:bg-gray-200 active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
          >
            Log In
          </button>
          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={goDashboard}
              className="w-full min-h-[44px] rounded-[8px] border border-gray-200 bg-white/70 px-4 py-2 font-semibold text-[#1D1D1F] transition-all hover:bg-white dark:border-white/10 dark:bg-white/10 dark:text-[#F5F5F7]"
            >
              Login
            </button>
            <button
              onClick={goDashboard}
              className="w-full min-h-[44px] rounded-[8px] border border-gray-200 bg-white/70 px-4 py-2 font-semibold text-[#1D1D1F] transition-all hover:bg-white dark:border-white/10 dark:bg-white/10 dark:text-[#F5F5F7]"
            >
              Sign in
            </button>
          </div>
        </div>

        <div className="relative flex items-center py-5">
            <div className="flex-grow border-t border-gray-300 dark:border-gray-600"></div>
            <span className="flex-shrink-0 mx-4 text-gray-400 dark:text-gray-500 text-sm">or</span>
            <div className="flex-grow border-t border-gray-300 dark:border-gray-600"></div>
        </div>

        <button
          onClick={() => router.push('/onboarding')}
          className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
        >
          Start Business Setup
        </button>
      </div>
    </div>
  );
}
