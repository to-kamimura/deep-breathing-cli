use clap::Parser;
use colored::*;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::{BufReader, Write, stdout},
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

/// Deep Breathing CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 吸う秒数
    #[arg(short, long)]
    inhale: u64,

    /// 吐く秒数
    #[arg(short, long)]
    exhale: u64,

    /// 繰り返し回数
    #[arg(short, long)]
    count: u64,

    /// 通常音ファイルパス
    #[arg(long, default_value = "sound/beep.mp3")]
    sound_file: PathBuf,

    /// フェーズ切り替え音ファイルパス
    #[arg(long, default_value = "sound/beep_period.mp3")]
    period_sound_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let (_stream, stream_handle) = OutputStream::try_default().expect("音声出力デバイスが見つかりません");

    for i in 1..=args.count {
        if i > 1 {
            clear_previous_set();
        }

        println!("セット {} / {}", i, args.count);

        println!("{}", "吸って...".green());
        visualize_breathing(
            args.inhale,
            BreathType::Inhale,
            &stream_handle,
            &args.sound_file,
            &args.period_sound_file,
        );

        println!("{}", "吐いて...".red());
        visualize_breathing(
            args.exhale,
            BreathType::Exhale,
            &stream_handle,
            &args.sound_file,
            &args.period_sound_file,
        );
    }

    println!("\n{}", "お疲れ様でした！".cyan());
}

/// フェーズ（吸う・吐く）
enum BreathType {
    Inhale,
    Exhale,
}

/// 呼吸動作を視覚＋音声で表現する
fn visualize_breathing(
    seconds: u64,
    breath_type: BreathType,
    stream_handle: &rodio::OutputStreamHandle,
    sound_path: &PathBuf,
    period_sound_path: &PathBuf,
) {
    for current in 1..=seconds {
        let filled = "●".repeat(current as usize);
        let empty = "◯".repeat((seconds - current) as usize);

        let display = match breath_type {
            BreathType::Inhale => format!("{}{}", filled, empty).green(),
            BreathType::Exhale => format!("{}{}", filled, empty).red(),
        };

        print!("\r{}", display);
        stdout().flush().unwrap();

        // 最後の1秒だけフェーズ切り替え音を鳴らす
        let file = if current == seconds {
            File::open(period_sound_path).expect("フェーズ終了音ファイルが開けません")
        } else {
            File::open(sound_path).expect("通常音ファイルが開けません")
        };

        let source = Decoder::new(BufReader::new(file)).expect("音声デコードに失敗しました");

        let sink = Sink::try_new(stream_handle).expect("Sink作成に失敗しました");
        sink.append(source);
        sink.detach(); // 非同期再生

        sleep(Duration::from_secs(1));
    }
    println!(); // 最後に改行
}

/// 前回のセット表示をすべて消す
fn clear_previous_set() {
    let lines_to_clear = 5; // セットタイトル + 吸うタイトル + 吸うゲージ + 吐くタイトル + 吐くゲージ

    for _ in 0..lines_to_clear {
        print!("\x1b[F"); // 1行上に戻る
        print!("\x1b[2K"); // 行を消す
    }
}
