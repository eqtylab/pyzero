import sys

# dump sys module info
for attr_name in dir(sys):
    attr_value = getattr(sys, attr_name)
    print(f"{attr_name}: {attr_value}")
