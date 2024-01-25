This repo tests the maximum number of `DefaultAtom` for the `string_cache` crate before significant performance degression.

## Results

| Size      | Avg Creation Time (ns) |
|-----------|------------------------|
| 100       | 47                     |
| 10,000    | 98                     |
| 100,000   | 290                    |
| 1,000,000 | 16,945                 |
