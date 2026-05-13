import { NextResponse } from 'next/server';
import { mockArticles } from '../articles/route';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const query = searchParams.get('q');

  if (!query) {
    return NextResponse.json([]);
  }

  const lowercaseQuery = query.toLowerCase();
  const results = mockArticles.filter(a =>
    a.title.toLowerCase().includes(lowercaseQuery) ||
    a.content.toLowerCase().includes(lowercaseQuery) ||
    a.category.toLowerCase().includes(lowercaseQuery)
  );

  return NextResponse.json(results);
}
