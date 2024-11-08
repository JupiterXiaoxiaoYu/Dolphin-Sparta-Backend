import { query, ZKWasmAppRpc, LeHexBN } from "zkwasm-ts-server";
const CMD_INSTALL_PLAYER = 1n;
const CMD_BUY_DOLPHIN = 16n;
const CMD_BUY_FOOD = 17n;
const CMD_BUY_MEDICINE = 18n;
const CMD_FEED_DOLPHIN = 19n;
const CMD_HEAL_DOLPHIN = 20n;
const CMD_ATTACK_EVIL_WHALE = 21n;
const CMD_BUY_POPULATION = 22n;
const CMD_COLLECT_COINS = 23n;
const CMD_ADD_COINS = 24n;
const CMD_SELL_DOLPHIN = 25n;
function createCommand(command) {
    return command << 32n;
}
export class Player {
    constructor(key, rpc) {
        this.processingKey = key;
        this.rpc = new ZKWasmAppRpc(rpc);
    }
    async getState() {
        let state = await this.rpc.queryState(this.processingKey);
        return JSON.parse(state.data);
    }
    async installPlayer() {
        let accountInfo = new LeHexBN(query(this.processingKey).pkx).toU64Array();
        try {
            let finished = await this.rpc.sendTransaction(new BigUint64Array([createCommand(CMD_INSTALL_PLAYER), accountInfo[1], accountInfo[2], 0n]), this.processingKey);
            console.log("Player installed at:", finished);
        }
        catch (e) {
            console.log("Install player error:", e);
        }
    }
    async sendGameCommand(command, param = 0) {
        let accountInfo = new LeHexBN(query(this.processingKey).pkx).toU64Array();
        try {
            const safeParam = BigInt(Math.min(Math.max(0, param), Number.MAX_SAFE_INTEGER));
            let finished = await this.rpc.sendTransaction(new BigUint64Array([
                createCommand(command), safeParam,
                accountInfo[1],
                accountInfo[2],
            ]), this.processingKey);
            console.log(`Game command ${command} executed with param ${safeParam}`);
            return this.getState();
        }
        catch (e) {
            console.log("Game command error:", e);
            throw e;
        }
    }
    async buyFood() {
        return this.sendGameCommand(CMD_BUY_FOOD);
    }
    async buyMedicine() {
        return this.sendGameCommand(CMD_BUY_MEDICINE);
    }
    async feedDolphin(dolphinId) {
        const state = await this.getState();
        if (!state.player.data.dolphins || !state.player.data.dolphins[dolphinId]) {
            throw new Error(`Dolphin with index ${dolphinId} does not exist`);
        }
        const dolphin = state.player.data.dolphins[dolphinId];
        console.log("Feeding dolphin:", dolphin);
        if (state.player.data.food_number <= 0) {
            throw new Error('Not enough food');
        }
        return this.sendGameCommand(CMD_FEED_DOLPHIN, Number(dolphin.id));
    }
    async healDolphin(dolphinId) {
        const state = await this.getState();
        if (!state.player.data.dolphins || !state.player.data.dolphins[dolphinId]) {
            throw new Error(`Dolphin with index ${dolphinId} does not exist`);
        }
        const dolphin = state.player.data.dolphins[dolphinId];
        console.log("Healing dolphin:", dolphin);
        if (state.player.data.medicine_number <= 0) {
            throw new Error('Not enough medicine');
        }
        return this.sendGameCommand(CMD_HEAL_DOLPHIN, Number(dolphin.id));
    }
    async attackEvilWhale() {
        return this.sendGameCommand(CMD_ATTACK_EVIL_WHALE);
    }
    async buyPopulation() {
        return this.sendGameCommand(CMD_BUY_POPULATION);
    }
    async collectCoins() {
        return this.sendGameCommand(CMD_COLLECT_COINS);
    }
    async addCoins() {
        return this.sendGameCommand(CMD_ADD_COINS);
    }
    async buyDolphin(dolphinType) {
        if (dolphinType < 0 || dolphinType > 2) {
            throw new Error('Invalid dolphin type. Must be 0 (Archer), 1 (Pikeman), or 2 (Warrior)');
        }
        return this.sendGameCommand(CMD_BUY_DOLPHIN, Number(dolphinType));
    }
    async sellDolphin(dolphinIndex) {
        const state = await this.getState();
        if (!state.player.data.dolphins || dolphinIndex >= state.player.data.dolphins.length) {
            throw new Error(`Invalid dolphin index: ${dolphinIndex}`);
        }
        const dolphin = state.player.data.dolphins[dolphinIndex];
        console.log("Attempting to sell dolphin:", {
            index: dolphinIndex,
            dolphin: dolphin,
            type: dolphin.name
        });
        return this.sendGameCommand(CMD_SELL_DOLPHIN, dolphinIndex);
    }
}
//# sourceMappingURL=api.js.map