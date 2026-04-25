The code review noted:
> However, the `SELECT` query and struct scanning omit the `Task ID` and `Dependencies` fields, which were explicitly requested in the "Data Fields to Synchronize" text section of the Design Doc.

Let's check if `AgentMission` struct has `TaskID` or `Dependencies`.
