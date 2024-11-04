import { query, ZKWasmAppRpc, LeHexBN } from "zkwasm-ts-server";
const CMD_INSTALL_PLAYER = 1n;
const CMD_GENERATE_RAND = 2n;
const CMD_REVEAL_RAND = 3n;
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
    async generateRand() {
        let accountInfo = new LeHexBN(query(this.processingKey).pkx).toU64Array();
        try {
            let finished = await this.rpc.sendTransaction(new BigUint64Array([createCommand(CMD_GENERATE_RAND), accountInfo[1], accountInfo[2], 0n]), this.processingKey);
            console.log("Random commitment generated at:", finished);
            return this.getState(); // 返回包含 commitment 的状态
        }
        catch (e) {
            console.log("Generate random error:", e);
        }
    }
    async revealRand(playerSignature) {
        let accountInfo = new LeHexBN(query(this.processingKey).pkx).toU64Array();
        try {
            let finished = await this.rpc.sendTransaction(new BigUint64Array([createCommand(CMD_REVEAL_RAND), accountInfo[1], accountInfo[2], playerSignature]), this.processingKey);
            console.log("Random revealed at:", finished);
            return this.getState(); // 返回包含最终随机数的状态
        }
        catch (e) {
            console.log("Reveal random error:", e);
        }
    }
    async generatePlayerSignature(commitment) {
        // 简单的签名实现：将commitment的两个值异或后与玩家的私钥异或
        const [c0, c1] = commitment;
        const commitmentHash = c0 ^ c1;
        return commitmentHash ^ BigInt(this.processingKey);
    }
    async requestRandomWithSignature() {
        // 1. 生成随机数承诺
        const state = await this.generateRand();
        if (!state?.seed_info?.commitment) {
            throw new Error("Failed to get commitment");
        }
        // 2. 获取commitment并转换为bigint数组
        const commitment = [
            BigInt(state.seed_info.commitment[0]),
            BigInt(state.seed_info.commitment[1])
        ];
        // 3. 生成签名
        const signature = await this.generatePlayerSignature(commitment);
        // 4. 使用签名揭示随机数
        return this.revealRand(signature);
    }
}
//# sourceMappingURL=api.js.map