# Dolphin-Sparta-Backend

# Dolphin-Sparta-Backend 项目结构

## src/ - 后端源代码
- `lib.rs` - 主入口文件
- `config.rs` - 配置文件，包含版本信息和游戏参数（modifiers尚未实现：有问题）
- `state.rs` - 状态管理，处理玩家状态和随机数生成（现在的ts对应的文件，目前仅用作随机数生成，有问题）
- `settlement.rs` - 结算相关逻辑
- `gameplay.rs` - 核心游戏玩法实现 - 海豚购买、喂食、治疗等功能（没有测试，没跑通）
- `random.rs` - 随机数生成和验证系统（state.rs的补充）  - 包含种子生成、承诺和验证逻辑
- `event.rs` - 事件系统（copy了automata，但是没有具体实现timetick和差分时间序列，不是非常能看懂）

## ts/ - 前端 TypeScript 代码
### src/
- `api.ts` - API 接口定义
  - 玩家类定义
  - 与后端交互的方法
- `test.ts` - 测试文件
  - 随机数生成测试
  - 玩家操作测试

##进度 && 需求
需要随机数生成模块用作抽卡，通过随机数来实现玩家购买海豚时的level和name随机（ENUM）
需要时间事件序列模块来管理成长、收获等内容
需要增加具体物件、海豚的Modifiers
需要跑通主要玩法的代码
