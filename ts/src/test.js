import { Player } from "./api.js";
// 创建玩家实例
const account = "2143524524545";
const player = new Player(account, "http://localhost:3000");
async function testGameplay() {
    try {
        // 1. 安装玩家
        await player.installPlayer();
        console.log("Player installed");
        let state = await player.getState();
        console.log("Initial state:", JSON.stringify(state, null, 2));
        if (state.player.data.dolphins.length > 0) {
            console.log("Dolphins already exist, skipping dolphin purchase");
        }
        else {
            // 2. 购买不同类型的海豚
            await player.buySpecificDolphin(0); // DolphinArcher
            state = await player.getState();
            console.log("After buying archer dolphin:", JSON.stringify(state, null, 2));
        }
        if (state.player.data.food_number <= 15) {
            // 3. 购买食物
            await player.buyFood();
            state = await player.getState();
            console.log("After buying food:", JSON.stringify(state, null, 2));
        }
        else {
            console.log("Food already enough, skipping food purchase");
        }
        await player.buySpecificDolphin(1); // DolphinPikeman
        state = await player.getState();
        console.log("After buying pikeman dolphin:", JSON.stringify(state, null, 2));
        // 4. 购买药品
        if (state.player.data.medicine_number <= 10) {
            await player.buyMedicine();
            state = await player.getState();
            console.log("After buying medicine:", JSON.stringify(state, null, 2));
        }
        else {
            console.log("Medicine already enough, skipping medicine purchase");
        }
        // 5. 喂食海豚
        const dolphinState = state.player.data.dolphins[0];
        console.log("Attempting to feed dolphin:", dolphinState);
        await player.feedDolphin(1); // 使用数组索引
        state = await player.getState();
        console.log("After feeding dolphin:", JSON.stringify(state, null, 2));
        // 6. 治疗海豚
        await player.healDolphin(0);
        state = await player.getState();
        console.log("After healing dolphin 1:", JSON.stringify(state, null, 2));
        // 7. 收集金币
        await player.collectCoins();
        state = await player.getState();
        console.log("After collecting coins:", JSON.stringify(state, null, 2));
        // 8. 添加金币
        await player.addCoins();
        state = await player.getState();
        console.log("After adding coins:", JSON.stringify(state, null, 2));
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