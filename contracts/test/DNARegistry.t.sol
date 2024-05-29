// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {DNARegistry} from "../src/DNARegistry.sol";

contract DNARegistryTest is Test {
    DNARegistry public registry;
    address user = makeAddr("user");
    bytes32 dnaRootHash = 0xe527cf6a0ba352e833a62375b2df4ae13e6f60e1d20d2ea06fe38751e99f4b44;

    function setUp() public {
        registry = new DNARegistry();
        registry.registerUserDNAHash(user, dnaRootHash);
    }
}
