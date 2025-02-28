# Global Admin #
| Функциональное назначение в бэкенде | Текущая функция (Rust) в клиенте |Роль пользователя|
|-------------------------------------|----------------------------------|----------------------------------|
| **Изменение параметров пула (комиссии, параметры)** | `CommandsName::UpdateConfig`, `update_amm_config_instr` |admin|
|  **Управление вознаграждениями (Reward)** | `CommandsName::InitReward`, `CommandsName::SetRewardParams`, `initialize_reward_instr`, `set_reward_params_instr` |admin|
| **Просмотр деталей конфига**           | `CommandsName::PConfig` |all|

# Pools Management #
| Функциональное назначение в бэкенде | Текущая функция (Rust) в клиенте |Роль пользователя|
|-------------------------------------|----------------------------------|----------------------------------|
| **Создание пула ликвидности**       | `CommandsName::CreatePool` и `create_pool_instr` |liquidity provider|
| **Просмотр деталей пула**           | `CommandsName::PConfig` |all|
| **Открытие новой позиции ликвидности** | `CommandsName::OpenPosition`, `open_position_with_token22_nft_instr` |liquidity provider|
| **Увеличение ликвидности в позиции**   | `CommandsName::IncreaseLiquidity`, `increase_liquidity_instr` |liquidity provider|
| **Уменьшение и закрытие позиции ликвидности** | `CommandsName::DecreaseLiquidity`, `decrease_liquidity_instr`, `close_personal_position_instr` |liquidity provider|
| **Просмотр списка позиций пользователя** | `CommandsName::PPositionByOwner`, `get_all_nft_and_position_by_owner` |all|
| **Расчёт параметров ликвидности и комиссии** | `CommandsName::LiquidityToAmounts`, `liquidity_math::*` |liquidity provider|

# Swap #
| Функциональное назначение в бэкенде | Текущая функция (Rust) в клиенте |Роль пользователя|
|-------------------------------------|----------------------------------|----------------------------------|
| **Своп токенов**                    | `CommandsName::Swap`, `CommandsName::SwapV2`, `swap_instr`, `swap_v2_instr` |all?|
| **Мониторинг текущих цен и состояния тиков** | `CommandsName::TickToPrice`, `CommandsName::PriceToTick`, `CommandsName::PTickState` |all|

# Quote #
Необходимо реализовать функционал для расчета котировки без симуляции свопа.
Входящие параметры для котировки:
```json
{
  "inputMint": "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R",
  "outputMint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "inputAmount": "100000000",
  "slippageBps": 50,
  "swapMode": "ExactIn",
}
```
1. Найти подходящий пул (по минт адресам токенов и перебора config_id)
2. Рассчитать количество выходного токена (либо входного если swapMode = ExactOut) и комиссию
3. Вернуть результат