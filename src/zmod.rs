use crate::args::PromptType;
use crate::zsh;
use once_cell::sync::OnceCell;
use std::ffi::{CString, c_void};
use std::os::raw::c_int;
use zsh_module::{Module, ModuleBuilder};
use zsh_sys::{addhookfunc, getiparam, hookdef, setsparam};

// 命名規則に従い大文字に変更
static TOKIO_RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::new();

pub fn setup() -> Result<Module, Box<dyn std::error::Error>> {
    // Runtimeの初期化（これを忘れるとプロンプトが更新されません）
    TOKIO_RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    });

    let module = ModuleBuilder::new(ZshInfiniteModule).build();
    unsafe {
        add_hooks();
    }
    Ok(module)
}

async fn left_prompt() -> String {
    zsh::build_prompt(&PromptType::Left).await.build()
}

async fn right_prompt() -> String {
    zsh::build_prompt(&PromptType::Right).await.build()
}

// 使われていない関数を一旦公開、または _ で対応
pub async fn hook_prompt() -> String {
    zsh::build_prompt(&PromptType::Hook).await.build()
}

pub async fn transient_prompt(exit_code: Option<i32>) -> String {
    zsh::build_prompt(&PromptType::Transient { exit_code })
        .await
        .build()
}

struct ZshInfiniteModule;
impl ZshInfiniteModule {}

unsafe extern "C" fn rust_precmd_hook(_: *mut hookdef, _: *mut c_void) -> c_int {
    if let Some(rt) = TOKIO_RUNTIME.get() {
        // unsafe関数の呼び出しをブロックで囲む
        let last_status = unsafe { getiparam(CString::new("?").unwrap().as_ptr() as *mut _) };

        // プロンプト生成に status を利用（未使用警告への対応）
        // 既存の PromptType に last_status を渡すロジックがある場合はここで調整してください
        let left = rt.block_on(left_prompt());
        let right = rt.block_on(right_prompt());

        unsafe {
            set_zsh_string("PROMPT", &left);
            set_zsh_string("RPROMPT", &right);
        }
    }
    0
}

unsafe fn set_zsh_string(name: &str, value: &str) {
    let name_c = CString::new(name).unwrap();
    let value_c = CString::new(value).unwrap();
    // 内部で再度unsafeブロックを使用
    unsafe {
        setsparam(name_c.as_ptr() as *mut _, value_c.as_ptr() as *mut _);
    }
}

unsafe fn add_hooks() {
    let hook_name = CString::new("precmd").unwrap();
    unsafe {
        addhookfunc(
            hook_name.as_ptr() as *mut _,
            Some(rust_precmd_hook as unsafe extern "C" fn(*mut hookdef, *mut c_void) -> c_int),
        );
    }
}
