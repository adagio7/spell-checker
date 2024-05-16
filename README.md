# Spell Checker

A Command-Line spell checker tool implemented with clap, which checks the spelling of words in text files and provides suggestions for misspelled words using different algorithms

## Running the application

```bash
    cargo run
```

## Roadmap

### Flags
- [ ] Implement file path flag
- [ ] Implement dictionary path flag
- [ ] Implement algorithm flag
- [ ] Implement suggestion count flag
- [ ] Implement verbosity flag

### Algorithms
- [ ] Implement the Levenshtein distance algorithm
- [ ] Implement BK-Tree algorithm
- [ ] Implement Norvig's algorithm

### Error Handling
- [ ] FileNotFound
- [ ] DictionaryNotFound
- [ ] InvalidAlgorithm
- [ ] InvalidSuggestionCount (should be greater than 0)

### Testing
- [ ] Unit tests
