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
    async sendGameCommand(command, dolphinId = 0) {
        let accountInfo = new LeHexBN(query(this.processingKey).pkx).toU64Array();
        try {
            let finished = await this.rpc.sendTransaction(new BigUint64Array([
                createCommand(command),
                accountInfo[1],
                accountInfo[2],
                BigInt(dolphinId)
            ]), this.processingKey);
            console.log(`Game command ${command} executed at:`, finished);
            return this.getState();
        }
        catch (e) {
            console.log("Game command error:", e);
            throw e;
        }
    }
    async buyDolphin() {
        return this.sendGameCommand(CMD_BUY_DOLPHIN);
    }
    async buyFood() {
        return this.sendGameCommand(CMD_BUY_FOOD);
    }
    async buyMedicine() {
        return this.sendGameCommand(CMD_BUY_MEDICINE);
    }
    async feedDolphin(dolphinId) {
        const state = await this.getState();
        if (!state.data.dolphins || !state.data.dolphins[dolphinId]) {
            throw new Error(`Dolphin with ID ${dolphinId} does not exist`);
        }
        if (state.data.food_number <= 0) {
            throw new Error('Not enough food');
        }
        return this.sendGameCommand(CMD_FEED_DOLPHIN, dolphinId);
    }
    async healDolphin(dolphinId) {
        return this.sendGameCommand(CMD_HEAL_DOLPHIN, dolphinId);
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
    async buySpecificDolphin(dolphinType) {
        return this.sendGameCommand(CMD_BUY_DOLPHIN, dolphinType);
    }
}
//# sourceMappingURL=api.js.map