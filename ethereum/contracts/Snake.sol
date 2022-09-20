// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "./interface/ISnake.sol";

contract Snake is ISnake {
    mapping(address => uint256[]) scores;
    Player[10] top10; // high to low

    event ScoreUploaded(address indexed account, uint256 score);
    event EnteredTop10(address indexed account, uint256 score, uint8 rank);

    function uploadScore(uint256 score) external override {
        scores[msg.sender].push(score);
        emit ScoreUploaded(msg.sender, score);
        insertTop(msg.sender, score);
    }

    function getMyScores() external view override returns (uint256[] memory) {
        return scores[msg.sender];
    }

    function getTop10() external view override returns (Player[10] memory) {
        return top10;
    }

    function insertTop(address account, uint256 score) internal {
        uint8 index;
        for (index = 0; index < 10; index++) {
            if (top10[index].score <= score) {
                break;
            }
        }
        if (index < 9) {
            for (uint8 j = 9; j > index; j--) {
                if (top10[j - 1].account != address(0)) {
                    top10[j] = top10[j - 1];
                }
            }
            top10[index].account = account;
            top10[index].score = score;
            emit EnteredTop10(account, score, index + 1);
        }
    }
}
