package main

import (
    "fmt"
    "go/ast"
    "go/parser"
    "go/token"
)

func main() {
    fset := token.NewFileSet()
    f, err := parser.ParseFile(fset, "srcs/server/orchestration/mesh.go", nil, 0)
    if err != nil {
        fmt.Println(err)
        return
    }

    for _, d := range f.Decls {
        if genDecl, ok := d.(*ast.GenDecl); ok && genDecl.Tok == token.TYPE {
            for _, spec := range genDecl.Specs {
                if typeSpec, ok := spec.(*ast.TypeSpec); ok {
                    fmt.Println(typeSpec.Name.Name)
                }
            }
        }
    }
}
