# Deep Breathing CLI

Rustで作成した、深呼吸トレーニング用のシンプルなコマンドラインツールです。  
吸う時間、吐く時間、セット回数を指定して、音と視覚的なガイドに従って呼吸を整えることができます。

## 機能

- 吸う秒数と吐く秒数を個別に設定可能
- 繰り返し回数（セット数）を指定可能
- `●`と`◯`で現在の呼吸進行状況を視覚表示
- 毎秒ごとにビープ音を再生
- フェーズ終了（吸い終わり、吐き終わり）時に異なるビープ音を再生
- 現在のセットのみをターミナルに表示（過去の表示は消去）

## 必要環境

- Rust（推奨バージョン: 1.70以降）
- Cargo（Rustに付属しています）

使用する外部クレート（依存ライブラリ）：

- [clap](https://crates.io/crates/clap) — コマンドライン引数パーサー
- [colored](https://crates.io/crates/colored) — ターミナル出力の色付け
- [rodio](https://crates.io/crates/rodio) — 音声再生ライブラリ

## インストールとビルド方法

まず、このリポジトリをクローンしてローカルにコピーします。

```bash
git clone https://github.com/あなたのユーザー名/deep-breathing-cli.git
cd deep-breathing-cli
```

リリースビルド（最適化版）を作成します。

```bash
cargo build --release
```

ビルド後、実行ファイルは以下に生成されます。

```
./target/release/deep-breathing-cli
```

## 使い方

以下のようにコマンドを実行します。

```bash
./target/release/deep_breath --inhale 4 --exhale 6 --count 3
```

### 利用できるオプション

| オプション名 | 説明 | 例 |
|:---|:---|:---|
| `--inhale` | 吸う時間（秒数） | `--inhale 4` |
| `--exhale` | 吐く時間（秒数） | `--exhale 6` |
| `--count` | セット回数 | `--count 3` |
| `--sound-file` | 毎秒鳴らす音声ファイルパス（デフォルト: `sound/beep.mp3`） |  |
| `--period-sound-file` | フェーズ終了時に鳴らす音声ファイルパス（デフォルト: `sound/beep_period.mp3`） |  |

### 音声ファイルについて

`sound/` フォルダ内に以下のファイルを準備してください。

- `beep.mp3` — 毎秒再生する通常ビープ音
- `beep_period.mp3` — 吸い終わり・吐き終わりに再生する特別なビープ音

（MP3形式、またはRodioがサポートする他のフォーマットでもOKです）

## ライセンス

このプロジェクトは [MITライセンス](LICENSE) のもとで提供されています。
