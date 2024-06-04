// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {SP1Verifier} from "@sp1-contracts/SP1Verifier.sol";

contract DNARegistry is SP1Verifier {
    address public owner;
    mapping(address => bytes32) public tempDnaHashes;
    mapping(address => bytes32) public dnaHashes;

    constructor() {
        owner = msg.sender;
    }

    function registerDNAHash(address user, bytes32 dnaHash) public {
        require(msg.sender == owner);
        tempDnaHashes[user] = dnaHash;
    }

    function approveDNAHash(bytes32 dnaHash) public {
        require(tempDnaHashes[msg.sender] == dnaHash);
        dnaHashes[msg.sender] = dnaHash;
    }

    function proveVariant(
        address user,
        uint8 target,
        uint index,
        bytes32 vkey,
        bytes memory proofBytes
    ) public {
        bytes memory publicValues = abi.encode(target, dnaHashes[user], index);
        verifyProof(vkey, publicValues, proofBytes);
    }
}
