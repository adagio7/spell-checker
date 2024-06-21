# Spell Checker

A Command-Line spell checker tool implemented with clap, which checks the spelling of words in text files and provides suggestions for misspelled words using different algorithms

## Status
- [x] COMPLETED

## Running the application

```bash
    cargo run -- [OPTIONS]
```

### Options

```bash
Options:
  -d, --dictionary_path <path>  Path to the dictionary file [default: ./dictionaries/google-10k-eng.txt]
  -t, --text_path <path>        Path to the text file to spell check
  -v, --verbose                 Prints debug information verbosely
  -n, --default_matches <n>     Number of default matches to return [default: 5]
  -m, --mode <mode>             Mode to run the spell checker in [default: levenshtein]
  -h, --help                    Print help
  -V, --version                 Print version          Print version
```

## Running tests

```bash
    cargo test
```

## Running linting

```bash
    cargo clippy
```

## Roadmap

### Flags
- [x] Implement file path flag
- [x] Implement dictionary path flag
- [x] Implement algorithm flag
- [x] Implement suggestion count flag
- [x] Implement verbosity flag

### Algorithms
- [x] Implement the Levenshtein distance algorithm
- [x] Implement BK-Tree algorithm
- [x] Implement Longest Common Subsequence (LCS) algorithm
- [x] Implement Hamming distance algorithm

### Error Handling
- [x] FileNotFound
- [x] DictionaryNotFound
- [x] InvalidAlgorithm
- [x] InvalidSuggestionCount (should be greater than 0)

### Testing
- [x] Unit tests
- [x] CI/CD pipeline
- [x] Integration tests

## Future Works

- Implement context-aware spell checking
- Implement a web interface for the spell checker
