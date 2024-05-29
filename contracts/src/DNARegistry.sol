// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract DNARegistry {
    address public owner;
    mapping(address => bytes32) public dnaHashes;

    constructor() {
        owner = msg.sender;
    }

    function registerUserDNAHash(address user, bytes32 dnaHash) public {
        dnaHashes[user] = dnaHash;
    }
}
