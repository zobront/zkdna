// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { DNARegistry } from "./DNARegistry.sol";

contract Labratory {
    struct Bounty {
        uint8 target;
        uint index;
        uint numParticipants;
        uint rewardPerParticipant;
        uint deadline;
        address[] participants;
        mapping(address => bool) hasParticipated;
    }

    DNARegistry immutable registry;
    mapping(bytes32 => Bounty) bounties;

    constructor(address _registry) {
        registry = DNARegistry(_registry);
    }

    function createBounty(uint8 target, uint index, uint numParticipants, uint deadline, uint nonce) public payable {
        uint rewardPerParticipant = msg.value / numParticipants;
        bytes32 bountyId = keccak256(abi.encodePacked(target, index, deadline, nonce));
        bounties[bountyId] = Bounty(target, index, numParticipants, rewardPerParticipant, deadline, new address[](0));
    }

    function participateInBounty(bytes32 bountyId, bytes32 vkey, bytes memory proofBytes) public {
        Bounty memory bounty = bounties[bountyId];

        require(!bounty.hasParticipated[msg.sender], "already participated");
        require(bounty.deadline > block.timestamp, "bounty expired");
        require(bounty.participants.length < bounty.numParticipants, "bounty complete");

        registry.proveVariant(msg.sender, bounty.target, bounty.index, vkey, proofBytes);

        bounty.participants.push(msg.sender);
        bounty.hasParticipated[msg.sender] = true;
        payable(msg.sender).transfer(bounty.rewardPerParticipant);

        bounties[bountyId] = bounty;
    }
}
