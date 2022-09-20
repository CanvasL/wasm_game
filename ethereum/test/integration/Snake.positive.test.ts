import { expect } from "chai";
import { ethers } from "hardhat";
import { BigNumber, ContractFactory, Contract } from "ethers";

describe("Snake(Positive)", async () => {
    let Snake: ContractFactory;
    let hardhatSnake: Contract;
    beforeEach(async()=>{
        Snake = await ethers.getContractFactory("Snake");
        hardhatSnake = await Snake.deploy();
    })

    describe("#1.Single player upload score", async () => {
        let owner: any;
        let score: BigNumber;
        beforeEach(async()=>{
            [owner] = await ethers.getSigners();
            score = BigNumber.from("100");
            await hardhatSnake.connect(owner).uploadScore(score);
        })

        it("get scores succeed", async () => {
            const myScores = await hardhatSnake.getMyScores();
            expect(myScores[0]).to.equal(score);
        });

        it("get top 10 players succeed", async () => {
            const top10Palyers = await hardhatSnake.getTop10();
            expect(top10Palyers[0].account).to.equal(owner.address);
            expect(top10Palyers[0].score).to.equal(score);
        });
    })
})
