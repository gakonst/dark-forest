pragma solidity ^0.8.2;

contract Proxy {
    bytes32 internal constant ADMIN_SLOT =  bytes32(uint256(keccak256("eip1967.proxy.admin")) - 1);
    bytes32 internal constant IMPLEMENTATION_SLOT = bytes32(uint256(keccak256("eip1967.proxy.implementation")) - 1);

    constructor(address owner, address _implementation) {
        getAddressSlot(ADMIN_SLOT).value = owner;
        getAddressSlot(IMPLEMENTATION_SLOT).value = _implementation;
    }

    struct AddressSlot {
        address value;
    }

    function getAddressSlot(bytes32 slot) internal pure returns (AddressSlot storage r) {
        assembly {
            r.slot := slot
        }
    }

    function implementation() public view returns (address) {
        return getAddressSlot(IMPLEMENTATION_SLOT).value;
    }

    function setImplementation(address who) public {
        require(msg.sender == admin());
        getAddressSlot(IMPLEMENTATION_SLOT).value = who;
    }

    function admin() public view returns (address) {
        return getAddressSlot(ADMIN_SLOT).value;
    }
    function setAdmin(address who) public {
        require(msg.sender == admin());
        getAddressSlot(ADMIN_SLOT).value = who;
    }

    function _delegate(address _implementation) private {
        assembly {
            calldatacopy(0, 0, calldatasize())
            let result := delegatecall(gas(), _implementation, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch result
            case 0 {
                revert(0, returndatasize())
            }
            default {
                return(0, returndatasize())
            }
        }
    }

    function _fallback() private {
        _delegate(implementation());
    }

    fallback() external payable {
        _fallback();
    }

    receive() external payable {
        _fallback();
    }

}
