import React from 'react';
import Link from 'next/link';

export function ArticleSections({ articles, hoverBg }: { articles: { category: string, title: string, desc: string, link: string }[], hoverBg: string }) {
  return (
    <div className="space-y-10 sm:space-y-12 flex flex-col">
      {Array.from(new Set(articles.map(a => a.category || "General"))).map((category) => (
        <section key={category} className="flex flex-col">
          <div className="flex items-center mb-4 sm:mb-6">
            <h2 className="text-xl sm:text-2xl font-bold font-outfit text-gray-900">{category}</h2>
            <div className="ml-4 flex-grow border-t border-gray-200/50"></div>
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 flex-col">
            {articles.filter(a => (a.category || "General") === category).map((article, idx) => (
              <Link key={idx} href={article.link} className="block group">
                <div className={`backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 p-5 sm:p-6 rounded-[24px] shadow-[0_8px_32px_rgba(0,0,0,0.08)] group-hover:border-blue-300 group-hover:shadow-[0_0_15px_rgba(59,130,246,0.5)] group-hover:-translate-y-1 transition-all duration-300 cursor-pointer h-full flex flex-col min-h-[120px] sm:min-h-[140px] ${hoverBg}`}>
                  <h3 className="text-lg sm:text-xl font-bold font-outfit text-blue-600 mb-2 sm:mb-3 group-hover:text-blue-700">{article.title}</h3>
                  <p className="text-sm sm:text-base text-gray-600 leading-relaxed flex-grow">{article.desc}</p>
                </div>
              </Link>
            ))}
          </div>
        </section>
      ))}
    </div>
  );
}
