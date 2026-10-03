import { expect,it } from 'vitest';
import fixture from './rust-definition-receipt.fixture.json';
import {readDefinition,readDefinitionReceipt,type Operation} from './definitions';

// Exact committed SQLite receipt produced by the actual Rust canonical-auth test
// scripts/agent-definition-contract/lifecycle_test.rs:
// unicode_newline_receipt_uses_the_actual_rust_digest_tuple.
// Source file SHA256:6b6703a72dfffb4d720b92741a89573ad612778be12d9433405e340462e7cb81.
const operation:Operation={kind:'publish',request_id:fixture.request_id,publication:{name:'研究者 🦀',description:'Line one\nQuoted "text" and café',role:'技術 writer',system_prompt:'Preserve\nUnicode 界 and slash / literally.\nNo tools.',visibility:'public'}};
it('verifies the exact Rust Unicode/newline publication digest and owner-bound receipt',async()=>{
 expect((await readDefinition(fixture.definition)).digest).toBe('8d8b6a0f3c8dd87ff0f4c16ae217c4a552f1123980681e7440311c8fe6ff1b93');
 expect(await readDefinitionReceipt(200,fixture,operation,{userId:fixture.user_id,tenantId:fixture.organization_id})).toEqual(fixture);
});
it('does not normalize changed Unicode content into the original Rust receipt',async()=>{
 await expect(readDefinition({...fixture.definition,description:fixture.definition.description.normalize('NFD')})).rejects.toThrow('changed');
});
