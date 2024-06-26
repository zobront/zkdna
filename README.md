# zkDNA

[23andme keeps getting hacked](https://techcrunch.com/2023/12/04/23andme-confirms-hackers-stole-ancestry-data-on-6-9-million-users/) 🤦🏻‍♂️

Genetic data for millions of users shouldn't be stored in a centralized database, but important scientific progress depends on being able to run experiments based on genetic variants.

zkDNA is a proof of concept for how to get the best of both worlds:
- A company like 23andme attests to a hash of your genetic data on chain
- They send you the data to store personally and delete it from their servers
- Researchers can provide bounties for users with specific variants participating in research
- Users participate in research and earn bounties by proving they have a specific variant (without revealing anything else about their genetic data)

zkDNA is built with [SP1](https://github.com/succinctlabs/sp1/).

### To Do

- [x] generate proofs for simple dna example
- [x] test E2E without zk
- [x] update parsing script to take in 23andme data format
- [x] patch tiny-merkle
- [x] non zk e2e using data input struct
- [x] get proof working in zkvm
- [x] create on chain components
- [ ] get plonk proofs working
- [ ] e2e example flow & record video walkthrough
- [ ] [sp1 keccak precompile](https://succinctlabs.github.io/sp1/writing-programs/precompiles.html)
- [ ] contracts: two step participate to validate survery completion with signature
