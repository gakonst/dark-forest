// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

interface DarkForestCore {
  function withdrawArtifact(uint256 locationId) external;
  function move(uint256[2] memory _a, uint256[2][2] memory _b, uint256[2] memory _c, uint256[7] memory _input) external returns (uint256);
  function planetEventsCount() external view returns (uint256);
  function findArtifact(uint256[2] memory _a, uint256[2][2] memory _b, uint256[2] memory _c, uint256[2] memory _input) external;
}

interface DarkForestTokens {
  function ownerOf(uint256 tokenId) external view returns (address);
  function transferFrom(address from, address to, uint256 tokenId) external;
  function safeTransferFrom(address from, address to, uint256 tokenId) external;
  function transferToCoreContract(uint256 tokenId) external;
}

contract Account {
  struct MoveProof {
    uint256[2] _a;
    uint256[2][2] _b;
    uint256[2] _c;
    uint256[3] _inputPart;
  }

  address payable private _owner;
  mapping(address => bool) allowedAccounts;

  DarkForestCore core;
  DarkForestTokens tokens;
  uint256 artifactRemovalLocationId;
  uint256 artifactRemovalIterationsLeft;

  mapping(uint256 => mapping(uint256 => MoveProof)) cachedMoveProofs;

  function onERC721Received(
    address,
    address from,
    uint256 tokenId,
    bytes calldata
  ) external returns (bytes4) {
    if (msg.sender == address(tokens) && from == address(core) && artifactRemovalIterationsLeft > 0) {
      artifactRemovalIterationsLeft--;
      tokens.transferFrom(address(this), address(core), tokenId);
      core.withdrawArtifact(artifactRemovalLocationId);
    }
    return 0x150b7a02;
  }

  receive() external payable {}

  function owner() public view returns (address) {
    return _owner;
  }

  function isAllowed(address account) public view returns (bool) {
    return _owner == account || allowedAccounts[account];
  }

  modifier onlyOwner() {
    require(_owner == msg.sender, "Ownable: caller is not the owner");
    _;
  }

  modifier onlyAllowed() {
    require(_owner == msg.sender || allowedAccounts[msg.sender] || address(this) == msg.sender, "Caller is not in Allowed");
    _;
  }

  function initV2(address payable newOwner) external {
    require(_owner == address(0), "Already initialized");
    _owner = newOwner;
  }

  function transferOwnership(address payable newOwner) external onlyOwner {
    require(newOwner != address(0), "Ownable: new owner is the zero address");
    _owner = newOwner;
  }

  function addAccount(address account) external onlyOwner {
    allowedAccounts[account] = true;
  }

  function removeAccount(address account) external onlyOwner {
    allowedAccounts[account] = false;
  }

  function forward(address payable target, bytes calldata payload) public payable onlyAllowed returns (bool, bytes memory) {
    return target.call{value: msg.value}(payload);
  }

  function forwardOrThrow(address payable target, bytes calldata payload) public payable onlyAllowed returns (bytes memory) {
    (bool success, bytes memory result) = target.call{value: msg.value}(payload);
    require(success, string(result));
    return result;
  }

  function setContracts(address _core, address _tokens) external onlyOwner {
    core = DarkForestCore(_core);
    tokens = DarkForestTokens(_tokens);
  }

  function removeArtifactRepeatedly(uint256 locationId, uint256 iterations) external onlyOwner {
    require(iterations >= 1, "Invalid number of iterations");
    require(address(core) != address(0) && address(tokens) != address(0), "contract addresses not set");
    require(artifactRemovalLocationId == 0 && artifactRemovalIterationsLeft == 0, "auxiliary variables not cleared");
    artifactRemovalLocationId = locationId;
    artifactRemovalIterationsLeft = iterations - 1;
    core.withdrawArtifact(locationId);
    require(artifactRemovalIterationsLeft == 0, "unexpected end state");
    artifactRemovalLocationId = 0;
  }

  function addCachedMoveProof(uint256[2] memory _a, uint256[2][2] memory _b, uint256[2] memory _c, uint256[5] memory _input) external onlyAllowed {
    cachedMoveProofs[_input[0]][_input[1]] = MoveProof(_a, _b, _c, [_input[2], _input[3], _input[4]]);
  }

  function hasCachedMoveProof(uint256 _oldLoc, uint256 _newLoc) external view returns (bool) {
    return cachedMoveProofs[_oldLoc][_newLoc]._inputPart[1] != 0;
  }

  function sendCachedMove(uint256 _oldLoc, uint256 _newLoc, uint256 _popMoved, uint256 _silverMoved) public onlyAllowed returns (uint256) {
    MoveProof memory proof = cachedMoveProofs[_oldLoc][_newLoc];
    require(proof._inputPart[1] != 0, "move proof not cached");
    uint256[7] memory _input = [_oldLoc, _newLoc, proof._inputPart[0], proof._inputPart[1], proof._inputPart[2], _popMoved, _silverMoved];
    return core.move(proof._a, proof._b, proof._c, _input);
  }

  function forwardToTargetPlanetEventsCount(uint256 _targetPlanetEventsCount, uint256 _oldLoc, uint256 _newLoc, uint256 _popMoved, uint256 _silverMoved) public onlyAllowed {
    uint256 planetEventsCount = core.planetEventsCount();
    require(_targetPlanetEventsCount >= planetEventsCount && _targetPlanetEventsCount - planetEventsCount <= 5, "unreachable _targetPlanetEventsCount");
    while (planetEventsCount < _targetPlanetEventsCount) {
      require(sendCachedMove(_oldLoc, _newLoc, _popMoved, _silverMoved) == planetEventsCount++, "unexpected planetEventsCount");
    }
  }

  function findArtifactWithTargetPlanetEventsCount(uint256[2] memory _a, uint256[2][2] memory _b, uint256[2] memory _c, uint256[2] memory _input, uint256 _targetPlanetEventsCount, uint256 _oldLoc, uint256 _newLoc, uint256 _popMoved, uint256 _silverMoved) external onlyAllowed {
    forwardToTargetPlanetEventsCount(_targetPlanetEventsCount, _oldLoc, _newLoc, _popMoved, _silverMoved);
    core.findArtifact(_a, _b, _c, _input);
  }
}
