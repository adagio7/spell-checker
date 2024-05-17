# Spell Checker

A Command-Line spell checker tool implemented with clap, which checks the spelling of words in text files and provides suggestions for misspelled words using different algorithms

## Running the application

```bash
    cargo run
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
- [ ] Implement BK-Tree algorithm
- [ ] Implement Norvig's algorithm

### Error Handling
- [ ] FileNotFound
- [x] DictionaryNotFound
- [x] InvalidAlgorithm
- [x] InvalidSuggestionCount (should be greater than 0)

### Testing
- [ ] Unit tests
