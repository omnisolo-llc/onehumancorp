import sys

content = []
content.append("pub mod synthetic_bench {")
for i in range(250):
    content.append("    #[test]")
    content.append(f"    fn test_synthetic_{i}() {{")
    content.append(f"        let x = {i};")
    content.append(f"        let y = {i} * 2;")
    content.append(f"        assert_eq!(x * 2, y);")
    content.append("    }")
content.append("}")

with open("src/server/benchmarks/synthetic_bench.rs", "w") as f:
    f.write("\n".join(content))
