pragma solidity ^0.8.13;

contract Rhuff {
  string public greeter = "RUSTISAGOODLANGUAGEFORACOMPILER.JPEG";

  function greet() public view returns (string memory) {
    return greeter;
  }
}