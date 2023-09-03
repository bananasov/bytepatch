# bytepatch
Lua bytecode patching program

## Defining a patch
It is recommended to use the following tree structure:
```
.
└── patches
    └── patch_name
        ├── fingerprint.lua
        ├── index.lua 
        └── patch.lua
```

TODO