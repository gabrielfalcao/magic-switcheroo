# magic-switcheroo

performs a magic number in the magic number of a file

[![CI](https://github.com/gabrielfalcao/magic-switcheroo/actions/workflows/main.yml/badge.svg)](https://github.com/gabrielfalcao/magic-switcheroo/actions/workflows/main.yml)

---

## Installation:

```bash
cargo install magic-switcheroo
```

## Example Usage


### Enchanting a file

```bash
ms e ice.ico --magic=AIRCONDIT
```

### Reversing the spell

just remember to use the same magic as in the previous case

```bash
ms r ice.ico --magic=AIRCONDIT
```

### Using raw-bytes as magic

Escape the bytes with hex-encoding

```bash
ms e Screenshot.png --magic=\x1c\xb0\x0d\xa\x370\x145
ms r Screenshot.png --magic=\x1c\xb0\x0d\xa\x370\x145
```
