import { Player } from "./api.js";

// 创建玩家实例
const account = "1234";
const player = new Player(account, "http://localhost:3000");

async function testGameplay() {
    try {
        // 1. 安装玩家
        await player.installPlayer();
        console.log("Player installed");
        let state = await player.getState();
        console.log("Initial state:", state);

        // 2. 购买不同类型的海豚
        await player.buySpecificDolphin(0); // DolphinArcher
        state = await player.getState();
        console.log("After buying archer dolphin:", state);

        await player.buySpecificDolphin(1); // DolphinPikeman
        state = await player.getState();
        console.log("After buying pikeman dolphin:", state);

        // 3. 购买食物
        await player.buyFood();
        state = await player.getState();
        console.log("After buying food:", state);

        // 4. 购买药品
        await player.buyMedicine();
        state = await player.getState();
        console.log("After buying medicine:", state);

        // 5. 喂食海豚
        await player.feedDolphin(0);
        state = await player.getState();
        console.log("After feeding dolphin 0:", state);

        // 6. 治疗海豚
        await player.healDolphin(1);
        state = await player.getState();
        console.log("After healing dolphin 1:", state);

        // 7. 收集金币
        await player.collectCoins();
        state = await player.getState();
        console.log("After collecting coins:", state);

        // 8. 攻击邪恶巨鲸
        await player.attackEvilWhale();
        state = await player.getState();
        console.log("After attacking evil whale:", state);

        // 9. 购买栏位
        await player.buyPopulation();
        state = await player.getState();
        console.log("After buying population slot:", state);

    } catch (error) {
        console.error("Test failed:", error);
    }
}

// 运行测试
testGameplay().catch(console.error);

