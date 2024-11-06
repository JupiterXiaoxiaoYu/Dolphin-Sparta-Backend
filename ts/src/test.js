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
        // 2. 购买海豚
        await player.buyDolphin();
        state = await player.getState();
        console.log("After buying dolphin:", state);
        // 3. 购买食物
        await player.buyFood();
        state = await player.getState();
        console.log("After buying food:", state);
        // 4. 购买药品
        await player.buyMedicine();
        state = await player.getState();
        console.log("After buying medicine:", state);
        // 5. 喂食海豚
        await player.feedDolphin(0); // 给 ID 为 0 的海豚喂食
        state = await player.getState();
        console.log("After feeding dolphin:", state);
        // 6. 治疗海豚
        await player.healDolphin(0); // 给 ID 为 0 的海豚治疗
        state = await player.getState();
        console.log("After healing dolphin:", state);
        // 7. 攻击邪恶巨鲸
        await player.attackEvilWhale();
        state = await player.getState();
        console.log("After attacking evil whale:", state);
        // 8. 购买栏位
        await player.buyPopulation();
        state = await player.getState();
        console.log("After buying population slot:", state);
    }
    catch (error) {
        console.error("Test failed:", error);
    }
}
// 运行测试
testGameplay().catch(console.error);
//# sourceMappingURL=test.js.map