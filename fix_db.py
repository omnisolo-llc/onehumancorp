with open("srcs/server/db/database.go", "r") as f:
    content = f.read()
if "ohc_memory_embeddings" not in content:
    # check where schema runs
    pass
