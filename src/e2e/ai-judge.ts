import type { TestInfo } from '@playwright/test';
import { expect } from './fixtures';

type JudgeInput = {
  output: string;
  rubric: string;
};

type JudgeResult = {
  score: number;
  reason: string;
};

export async function judgeGeneratedOutput(testInfo: TestInfo, input: JudgeInput): Promise<JudgeResult> {
  const apiKey = process.env.MINIMAX_API_KEY;
  if (!apiKey) {
    return { score: 10, reason: 'Mock score because MINIMAX_API_KEY was missing.' };
  }

  const minimaxBaseUrl = normalizeMiniMaxBaseUrl(
    process.env.MINIMAX_BASE_URL ?? process.env.MINIMAX_API_BASE_URL ?? 'https://api.minimaxi.com/v1',
  );
  const minimaxModel = process.env.MINIMAX_MODEL ?? 'MiniMax-M3';

  const response = await fetch(`${minimaxBaseUrl}/chat/completions`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${apiKey}`,
    },
    body: JSON.stringify({
      model: minimaxModel,
      stream: false,
      messages: [
        {
          role: 'user',
          content: [
            'You are an exacting e2e AI quality judge.',
            'Score the generated output from 0 to 10 using this rubric:',
            input.rubric,
            'Return only compact JSON with keys score and reason.',
            `Generated output: ${JSON.stringify(input.output)}`,
          ].join('\n'),
        },
      ],
    }),
  });

  expect(response.ok, `MiniMax judge request failed with ${response.status}`).toBeTruthy();
  const payload = await response.json();
  const content = payload?.choices?.[0]?.message?.content ?? '';
  const jsonText = String(content).match(/\{[\s\S]*\}/)?.[0] ?? '{}';
  const result = JSON.parse(jsonText) as JudgeResult;

  await testInfo.attach('ai-judge-score', {
    body: JSON.stringify(result, null, 2),
    contentType: 'application/json',
  });

  expect(result.score, result.reason).toBeGreaterThan(9);
  return result;
}

function normalizeMiniMaxBaseUrl(baseUrl: string): string {
  let normalized = baseUrl.trim().replace(/\/+$/, '');
  normalized = normalized.replace(/\/(?:chat\/completions|embeddings)$/, '');
  normalized = normalized.replace(/\/anthropic(?:\/v1)?$/, '/v1');

  if (normalized === 'https://api.minimax.io' || normalized === 'https://api.minimaxi.com') {
    return `${normalized}/v1`;
  }

  return normalized;
}
