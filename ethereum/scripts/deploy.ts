import { ethers } from "hardhat";

async function main() {
  const Snake = await ethers.getContractFactory("Snake");
  const snake = await Snake.deploy();

  await snake.deployed();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
