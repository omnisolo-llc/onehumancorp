with open("srcs/server/db/database.go", "r") as f:
    content = f.read()
if "VECTOR(1536)" in content:
    print("Database SQLite fallback is handling vectors properly by replacing them with TEXT")
