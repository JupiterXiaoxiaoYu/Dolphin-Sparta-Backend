# Dolphin-Sparta-Backend

## src/ - 后端源代码
- `lib.rs` - 主入口文件
- `config.rs` - 配置文件，包含版本信息和游戏参数
- `state.rs` - 状态管理，处理玩家状态和随机数生成
- `settlement.rs` - 结算相关逻辑
- `gameplay.rs` - 核心游戏玩法实现 - 海豚购买、喂食、治疗等功能
- `random.rs` - 随机数生成和验证系统
- `event.rs` - 事件系统

## ts/ - 前端 TypeScript 代码
### src/
- `api.ts` - API 接口定义
  - 玩家类定义
  - 与后端交互的方法
- `test.ts` - 测试文件
  - 随机数生成测试
  - 玩家操作测试

