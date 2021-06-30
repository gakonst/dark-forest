pragma solidity 0.8.5;

import "./Proxy.sol";
import "./Create2.sol";

contract AccountFactory {
    function make(address implementation, bytes32 salt) public returns (address) {
        return Create2.deploy(
            0,
            salt,
            abi.encodePacked(
                type(Proxy).creationCode,
                abi.encode(implementation, msg.sender)
            )
        );
    }
}
