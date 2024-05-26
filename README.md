# Spell Checker

A Command-Line spell checker tool implemented with clap, which checks the spelling of words in text files and provides suggestions for misspelled words using different algorithms

## Running the application

```bash
    cargo run -- [OPTIONS]
```

### Options

Options:
  -d, --dictionary_path <path>  Path to the dictionary file [default: ./dictionaries/google-10k-eng.txt]
  -v, --verbose                 Prints debug information verbosely
  -n, --default_matches <n>     Number of default matches to return [default: 5]
  -m, --mode <mode>             Mode to run the spell checker in [default: levenshtein]
  -h, --help                    Print help
  -V, --version                 Print version

## Running tests

```bash
    cargo test
```

## Roadmap

### Flags
- [ ] Implement file path flag
- [x] Implement dictionary path flag
- [x] Implement algorithm flag
- [x] Implement suggestion count flag
- [x] Implement verbosity flag

### Algorithms
- [x] Implement the Levenshtein distance algorithm
- [x] Implement BK-Tree algorithm
- [ ] Implement Norvig's algorithm

### Error Handling
- [ ] FileNotFound
- [x] DictionaryNotFound
- [x] InvalidAlgorithm
- [x] InvalidSuggestionCount (should be greater than 0)

### Testing
- [ ] Unit tests
- [ ] CI/CD pipeline
