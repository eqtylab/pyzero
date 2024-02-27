import builtins

builtin_functions = [name for name in dir(builtins) if callable(getattr(builtins, name))]

print(builtin_functions)

