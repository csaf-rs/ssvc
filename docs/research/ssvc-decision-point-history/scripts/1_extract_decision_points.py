import ast, os, sys

# Usage: python3 1_extract_decision_points.py [path-to-ssvc-checkout]
# Run from (or point at) a checkout of https://github.com/CERTCC/SSVC
REPO_DIR = sys.argv[1] if len(sys.argv) > 1 else "."
ROOT = os.path.join(REPO_DIR, "src/ssvc/decision_points")
results = []  # (namespace, file, varname, name, key, version)

for dirpath, dirnames, filenames in os.walk(ROOT):
    if "__pycache__" in dirpath:
        continue
    for fn in filenames:
        if not fn.endswith(".py"):
            continue
        if fn in ("__init__.py", "base.py", "helpers.py", "_not_defined.py"):
            continue
        path = os.path.join(dirpath, fn)
        rel = os.path.relpath(path, ROOT)
        namespace = rel.split(os.sep)[0] if os.sep in rel else "."
        with open(path) as f:
            src = f.read()
        try:
            tree = ast.parse(src, filename=path)
        except SyntaxError as e:
            print("SYNTAX ERROR", path, e, file=sys.stderr)
            continue
        for node in tree.body:
            if isinstance(node, ast.Assign) and isinstance(node.value, ast.Call):
                call = node.value
                funcname = ""
                if isinstance(call.func, ast.Name):
                    funcname = call.func.id
                elif isinstance(call.func, ast.Attribute):
                    funcname = call.func.attr
                if "DecisionPoint" not in funcname or funcname == "DecisionPointValue":
                    continue
                kwargs = {}
                for kw in call.keywords:
                    if kw.arg and isinstance(kw.value, ast.Constant):
                        kwargs[kw.arg] = kw.value.value
                if "version" not in kwargs:
                    continue
                varname = None
                if len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
                    varname = node.targets[0].id
                relpath = os.path.relpath(path, REPO_DIR)
                results.append((namespace, relpath, varname, kwargs.get("name"), kwargs.get("key"), kwargs.get("version")))

for r in results:
    print("|".join(str(x) for x in r))
print(f"TOTAL: {len(results)}", file=sys.stderr)
