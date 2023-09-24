# Runtime

## JSON ENCODING SCHEME
From the ink project

```
Glue:           "<>", "G<", "G>"

ControlCommand: "ev", "out", "/ev", "du" "pop", "->->", "~ret", "str", "/str", "nop", 
                "choiceCnt", "turns", "visit", "seq", "thread", "done", "end"

NativeFunction: "+", "-", "/", "*", "%" "~", "==", ">", "<", ">=", "<=", "!=", "!"... etc

Void:           "void"

Value:          "^string value", "^^string value beginning with ^"
                5, 5.2
                {"^->": "path.target"}
                {"^var": "varname", "ci": 0}

Container:      [...]
                [..., 
                    {
                        "subContainerName": ..., 
                        "#f": 5,                    // flags
                        "#n": "containerOwnName"    // only if not redundant
                    }
                ]

Divert:         {"->": "path.target", "c": true }
                {"->": "path.target", "var": true}
                {"f()": "path.func"}
                {"->t->": "path.tunnel"}
                {"x()": "externalFuncName", "exArgs": 5}

Var Assign:     {"VAR=": "varName", "re": true}   // reassignment
                {"temp=": "varName"}

Var ref:        {"VAR?": "varName"}
                {"CNT?": "stitch name"}

ChoicePoint:    {"*": pathString,
                    "flg": 18 }

Choice:         Nothing too clever, it's only used in the save state,
                there's not likely to be many of them.

Tag:            {"#": "the tag text"}
```