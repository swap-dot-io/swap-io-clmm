# Участники процесса деплоя Swap.io - clmm
1. *Initial deployer* - деплоер, который создает буфер и загружает в него программу. У него на балансе должно быть досаточно SOL для создания буфера, на 2025-03-12 - **9 SOL**. Может деплоить из любого окружения.
2. *Final deployer* - деплоер, который закрывает буфер и деплоит программу. У него на балансе должно быть досаточно SOL для создания program account, **~0.001151 SOL**. Должен деплоить **из защищенного окружения**. Перед деплоем удостовериться что в буфере находится именно та программа, которую нужно деплоить.
3. *Admin* - не участвует в процессе деплоя, но в дальнейшем управляет конфигурацией программы. Должен находиться **в защищенном окружении**.

# Необходимые Keypairs
1. *Buffer keypair* - ключ для создания буфера. Создается командой `solana-keygen new -o buffer.json`, можно переиспользовать в разных деплоях. Находится в любом окружении.
2. *Program keypair* - ключ для создания program account (**SWPammPnp7L9qFgV436u3CSPmcxU6ZQm6ttawzDTRuw**). Находится **в защищенном окружении**.

# Необходимые данные
 | Keypair | Окружение | Комментарий | mainnet |
 | --- | --- | --- | --- |
 |Initial deployer|Любое|~9 SOL|3xwmXSrn5do7TDWFXTLaN8Y9fxg67dXUYqbFi1VffR8V|
 |Buffer keypair|Любое| |4fmFw6zhkou96qnGm88PFtkarHeybHQM4qaGLzVcYgj1|
 |Final deployer|Защищенное|~0.001151 SOL|9AHfnBcPBTMZLvF9K5s6KT62d6naNjjJLJ6CMncboQkR|
 |Admin|Защищенное|чуть-чуть SOL |F92EBvFBAXkcYfjrdXSNmjbmpoK1pPXLtxuZPWoJPSde-|
 |Program keypair|Защищенное| |SWPammPnp7L9qFgV436u3CSPmcxU6ZQm6ttawzDTRuw|


# Devnet deploy
*Initial deployer: 3xwmXSrn5do7TDWFXTLaN8Y9fxg67dXUYqbFi1VffR8V*
```bash
solana config set -k /root/.config/solana/id.json
```
*Final deployer: DyzhJPZrma1wWWewf47QdAviXqBuAAWKwwSkRH3teT8W (local)| AuFC7TMFUZJiR4vcNUrMnYGUVqHU53WGWqvh5VUMQrps (devnet) | 9AHfnBcPBTMZLvF9K5s6KT62d6naNjjJLJ6CMncboQkR (mainnet)*

```bash
solana config set -k /root/.config/solana/payer.json
```

*buffer: 4fmFw6zhkou96qnGm88PFtkarHeybHQM4qaGLzVcYgj1*


# DEPLOYMENT
## Write Buffer command. Выполняем от имени initial deployer

```bash
solana program write-buffer ./target/deploy/swap_io_clmm.so --buffer ./buffer.json -k /root/.config/solana/id.json
```

## Set Buffer Authority command. Выполняем от имени initial deployer

```bash
solana program set-buffer-authority 4fmFw6zhkou96qnGm88PFtkarHeybHQM4qaGLzVcYgj1 -k /root/.config/solana/id.json --new-buffer-authority AuFC7TMFUZJiR4vcNUrMnYGUVqHU53WGWqvh5VUMQrps
```

## Prepare finalization transaction. Выполняем от имени final deployer (заменить public keys на актуальные)

```json
{
    "recentBlockhash": null,
    "feePayer": null,
    "nonceInfo": null,
    "instructions": [
        {
            "keys": [
                {
                    "pubkey": "AuFC7TMFUZJiR4vcNUrMnYGUVqHU53WGWqvh5VUMQrps",
                    "isSigner": true,
                    "isWritable": true
                },
                {
                    "pubkey": "6MynR3j7PgAMyUjbU3qgKz38dnR4CqPbXQEbxedKdiCQ",
                    "isSigner": true,
                    "isWritable": true
                }
            ],
            "programId": "11111111111111111111111111111111",
            "data": [
                0,
                0,
                0,
                0,
                24,
                144,
                17,
                0,
                0,
                0,
                0,
                0,
                36,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                2,
                168,
                246,
                145,
                78,
                136,
                161,
                176,
                226,
                16,
                21,
                62,
                247,
                99,
                174,
                43,
                0,
                194,
                185,
                61,
                22,
                193,
                36,
                210,
                192,
                83,
                122,
                16,
                4,
                128,
                0,
                0
            ]
        },
        {
            "keys": [
                {
                    "pubkey": "AuFC7TMFUZJiR4vcNUrMnYGUVqHU53WGWqvh5VUMQrps",
                    "isSigner": true,
                    "isWritable": true
                },
                {
                    "pubkey": "D3sXvWsFPsF3K4JjkU1MM2KVrFMF9vL8AQAYJvXvGMFo",
                    "isSigner": false,
                    "isWritable": true
                },
                {
                    "pubkey": "6MynR3j7PgAMyUjbU3qgKz38dnR4CqPbXQEbxedKdiCQ",
                    "isSigner": false,
                    "isWritable": true
                },
                {
                    "pubkey": "4fmFw6zhkou96qnGm88PFtkarHeybHQM4qaGLzVcYgj1",
                    "isSigner": false,
                    "isWritable": true
                },
                {
                    "pubkey": "SysvarRent111111111111111111111111111111111",
                    "isSigner": false,
                    "isWritable": false
                },
                {
                    "pubkey": "SysvarC1ock11111111111111111111111111111111",
                    "isSigner": false,
                    "isWritable": false
                },
                {
                    "pubkey": "11111111111111111111111111111111",
                    "isSigner": false,
                    "isWritable": false
                },
                {
                    "pubkey": "AuFC7TMFUZJiR4vcNUrMnYGUVqHU53WGWqvh5VUMQrps",
                    "isSigner": true,
                    "isWritable": false
                }
            ],
            "programId": "BPFLoaderUpgradeab1e11111111111111111111111",
            "data": [
                2,
                0,
                0,
                0,
                136,
                138,
                18,
                0,
                0,
                0,
                0,
                0
            ]
        }
    ],
    "signers": []
}
```
