# z3r-sramr

# Installation

`pip install z3r-sramr-py`

# Functions

`parse_sram(sram, validate_sram=true)`

Accepts a bytes object and optional boolean argument. If you want to skip the SRAM validation
checks, you can pass `False` as the second argument. Returns a dict with three
sub dicts: `'meta'`, `'stats'`, and `'equipment'`. See keys.txt for a list of all
keys.

Can raise a ParseException if one of the validation checks fail or another error
occurs.

`validate_sram(sram)`

Accepts a bytes object. Returns `True` if the object is valid ALTTPR SRAM and `False` if one
of the checks fail. The `parse_sram()` function calls this function by default.

# Example

```python
from z3rsramr import parse_sram

with open("example.srm", "rb") as f:
    sram = f.read()

sram_map = parse_sram(sram)

print(f'Current Rupees: {sram_map['equipment']['current rupees']}')
print(f'Collection Rate: {sram_map['stats']['collection rate']}')
print(f'Menu Time: {sram_map['stats']['menu time']}')
```
