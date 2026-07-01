import { NextResponse } from 'next/server';

export async function GET() {
  const mockEvents = [
    {
      type: 'RunStarted',
      iteration: 0,
    },
    {
      type: 'TextChunk',
      content: 'Starting Phase: Gather. Let me use the structured_output tool.',
    },
    {
      type: 'ToolCall',
      name: 'structured_output',
      args_json: '{"wrong_key": "value"}',
      result: 'Validation Error (Pydantic-first tool schema): Missing required \'data\' parameter in tool call arguments. Please include the data matching the schema inside the \'data\' property and retry calling the tool.',
      iteration: 0,
      isLlmRecoverable: true,
    },
    {
      type: 'TextChunk',
      content: 'I made a mistake. Let me correct the schema structure by nesting it inside the data key as instructed.',
    },
    {
      type: 'ToolCall',
      name: 'structured_output',
      args_json: '{"data": {"city": "Tokyo", "population": 14000000}}',
      result: 'Success',
      iteration: 1,
      isLlmRecoverable: false,
    },
    {
      type: 'TaskComplete',
      content: 'The parsing was successful and the data has been saved.',
    }
  ];

  return NextResponse.json(mockEvents, { status: 200 });
}
