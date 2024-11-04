//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { Player } from "./api.js";
let account = "1234";
let player = new Player(account, "http://localhost:3000");
async function main() {
  //let towerId = 10038n + y;

  await player.installPlayer();
  console.log("Player installed");

  // 3. 使用封装好的一键式随机数生成流程
  const finalState = await player.requestRandomWithSignature();
  console.log("Final state:", finalState);
  console.log("Final random number:", finalState.final_random);
}

main();

