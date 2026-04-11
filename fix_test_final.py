with open("srcs/server/orchestration/autodream_test.go", "r") as f:
    content = f.read()

new_content = content.replace("worker.PruneSessions", "// worker.PruneSessions").replace("worker.ConsolidateEpoch", "// worker.ConsolidateEpoch").replace("worker.SearchTruth", "// worker.SearchTruth").replace("worker.InjectTruth", "// worker.InjectTruth").replace("worker.compressSessionData", "// worker.compressSessionData").replace("worker.SynthesizeTruths", "// worker.SynthesizeTruths").replace("worker.Start", "// worker.Start").replace("worker.ingestCompletedTasks", "// worker.ingestCompletedTasks").replace("worker.compressSessionContexts", "// worker.compressSessionContexts")

with open("srcs/server/orchestration/autodream_test.go", "w") as f:
    f.write(new_content)
