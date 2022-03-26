# Multiplicity Persistence

## Usage

```bash
$ cargo run

Enter number of digits to test (e.g. 10_, 2d, 3c, 44k, 55M): 7_
Enter the minimum multiplicity to log the number (e.g. 2, 3, 9, 11): 5
🟠 About to test 314722122001 numbers...
🔵 Multiplicity = 6, Number = 2222378
```

## Notes

- Maximum number possible to represent by `u32`: `2^32 - 1 = 4294967295` (10 digits)
- Maximum number possible to represent by `u64`: `2^64 - 1 = 18446744073709551615` (20 digits)
- Maximum number possible to represent by `u128`: `2^128 - 1 = 340282366920938463463374607431768211455` (40 digits)

- Digits to avoid: `1`, `5`, and `0`
