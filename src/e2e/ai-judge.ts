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

  const response = await fetch('https://api.minimax.chat/v1/chat/completions', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${apiKey}`,
    },
    body: JSON.stringify({
      model: 'MiniMax-M2.7',
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
