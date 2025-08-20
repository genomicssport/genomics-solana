# genomicsSOL

![](https://github.com/genomicssport/genomics-solana/blob/main/image.jpg)

- Implementing the RUST blockchain based Health care management for the variant annotation and search.
- Annotate and claim your sequenced variants using the Blockchain Healthcare.
- Annotate with in a flick of a second 23&me personal genomics and generate SOL tokens for personal genomics.
- This annotates your ancestral and personal variants in a flash of a second and generates blockchain Solana token so that you can sell your ancestry.

```
SolGenome.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      ************************************************

Usage: genomics-solana <COMMAND>

Commands:
  generate-sol
  generate-info
  annotate-all
  rs-id-solana
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
./target/debug/genomics-solana generate-sol ./test-samples/variantfile.txt rs28444699
"GenomicPlacements"
"83SubSNP,15Frequencysubmissions"

./target/debug/genomics-solana generate-info rs548049170
"GenomicPlacements"
"Gene-OR4F5-,olfactoryreceptorfamily4subfamilyFmember5(plusstrand)"
"8SubSNP,3Frequencysubmissions"

./target/debug/genomics-solana rs-id-solana rs548049170
```

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland
