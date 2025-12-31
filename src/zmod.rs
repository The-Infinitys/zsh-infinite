use std::sync::atomic::{AtomicUsize, Ordering};

use zsh_system::{Features, Hook, ZshModule, ZshResult, export_module, zsh_hook_handler};

#[derive(Default)]
struct ZshInfinite {}
// 実行回数を保持するための静的変数（スレッドセーフ）
static PRECMD_COUNT: AtomicUsize = AtomicUsize::new(0);

zsh_hook_handler!(my_precmd_logger, _context, {
    let count = PRECMD_COUNT.fetch_add(1, Ordering::SeqCst);

    // 確実に目立たせるために、大量の改行とベル( \x07 )を鳴らす
    // また、標準エラー出力をフラッシュする
    use std::io::{Write, stderr};
    let mut s = stderr();
    let _ = writeln!(s, "\x07\r\n!!! HOOK CALLED #{} !!!\r\n", count + 1);
    let _ = s.flush();

    0
});

impl ZshModule for ZshInfinite {
    fn setup(&mut self) -> ZshResult {
        println!("ZshInfinite: setup...");
        Ok(())
    }

    fn boot(&mut self) -> ZshResult {
        // precmd だけでなく exit (シェル終了時) にも登録
        Hook::add("precmd", my_precmd_logger)?;
        Hook::add("exit", my_precmd_logger)?;
        eprintln!("[ZshInfinite] Hooks 'precmd' and 'exit' registered.");
        Ok(())
    }

    fn features(&self) -> Features {
        // 3. ビルトインコマンドの登録
        Features::new()
            .add_builtin("hello_infinite", |name, args| {
                println!("Greetings from {}!", name);
                println!("Arguments passed: {:?}", args);
                0 // 成功
            })
            .add_builtin("rust_status", |_, _| {
                println!("Rust core is running smoothly.");
                0
            })
    }

    fn cleanup(&mut self) -> ZshResult {
        let _ = Hook::remove("precmd", my_precmd_logger);
        let _ = Hook::remove("exit", my_precmd_logger);
        Ok(())
    }

    fn finish(&mut self) -> ZshResult {
        Ok(())
    }
}

// マクロでエクスポート
export_module!(ZshInfinite);
