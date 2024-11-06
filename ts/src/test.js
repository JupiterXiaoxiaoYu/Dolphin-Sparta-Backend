import { Player } from "./api.js";
// 创建玩家实例
const account = "12344";
const player = new Player(account, "http://localhost:3000");
async function testGameplay() {
    try {
        // 1. 安装玩家
        await player.installPlayer();
        console.log("Player installed");
        let state = await player.getState();
        console.log("Initial state:", JSON.stringify(state, null, 2));
        // 2. 购买不同类型的海豚
        await player.buySpecificDolphin(0); // DolphinArcher
        state = await player.getState();
        console.log("After buying archer dolphin:", JSON.stringify(state, null, 2));
        await player.buySpecificDolphin(1); // DolphinPikeman
        state = await player.getState();
        console.log("After buying pikeman dolphin:", JSON.stringify(state, null, 2));
        // 3. 购买食物
        await player.buyFood();
        state = await player.getState();
        console.log("After buying food:", JSON.stringify(state, null, 2));
        // 4. 购买药品
        await player.buyMedicine();
        state = await player.getState();
        console.log("After buying medicine:", JSON.stringify(state, null, 2));
        // 5. 喂食海豚
        const dolphinId = state.player.data.dolphins[0].id;
        await player.feedDolphin(dolphinId);
        state = await player.getState();
        console.log(`After feeding dolphin ${dolphinId}:`, JSON.stringify(state, null, 2));
        // 6. 治疗海豚
        await player.healDolphin(1);
        state = await player.getState();
        console.log("After healing dolphin 1:", JSON.stringify(state, null, 2));
        // 7. 收集金币
        await player.collectCoins();
        state = await player.getState();
        console.log("After collecting coins:", JSON.stringify(state, null, 2));
        // 8. 攻击邪恶巨鲸
        await player.attackEvilWhale();
        state = await player.getState();
        console.log("After attacking evil whale:", JSON.stringify(state, null, 2));
        // 9. 购买栏位
        await player.buyPopulation();
        state = await player.getState();
        console.log("After buying population slot:", JSON.stringify(state, null, 2));
    }
    catch (error) {
        console.error("Test failed:", error);
    }
}
// 运行测试
testGameplay().catch(console.error);
//# sourceMappingURL=test.js.map