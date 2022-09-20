// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;
import "./IStruct.sol";

interface ISnake is IStruct {
    function uploadScore(uint256 score) external;

    function getMyScores() external view returns (uint256[] memory);

    function getTop10() external view returns (Player[10] memory);
}
