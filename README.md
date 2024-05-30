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

- [x] test E2E without zk
- [ ] get proof working in zkvm
- [ ] create on chain components
- [ ] create build scripts for e2e example]
