#!/bin/zsh

# --- 基本設定 ---
# set は引数なしだと全変数を表示してしまうので、オプション設定なら setopt を使います。
# ここでは PROMPT_SUBST をオフにすることを明示します。
unsetopt PROMPT_SUBST
ZLE_RPROMPT_INDENT=0

# --- カーソルリセット ---
function _reset_infinite_cursor() {
    # echo ではなく printf を使い、出力を確実にターミナル制御に向けます
    printf '\e[0 q'
}

# --- プロンプト更新フック ---
function _update_infinite_prompt() {
    export LAST_STATUS=$?
    _reset_infinite_cursor
    # ビルトイン実行時の標準出力と標準エラー出力を両方破棄
    __zsh_infinite_internal_precmd >/dev/null 2>&1
}

# --- Transient Prompt ---
function _infinite_transient_prompt() {
    export LAST_STATUS=$?
    export LAST_COMMAND_EXECUTED=$EPOCHREALTIME
    
    # ここも外部プロセス呼び出しなので、エラーが混じらないようガード
    PROMPT="$(zsh-infinite zsh prompt transient --exit-code=${LAST_STATUS} 2>/dev/null)"
    RPROMPT=""
    
    zle reset-prompt 2>/dev/null
}

{
    autoload -Uz add-zsh-hook
    add-zsh-hook precmd _update_infinite_prompt

    # 現在の zle-line-finish の状態をチェック
    # $widgets[zle-line-finish] には "user:_関数名" や "builtin" といった形式で格納されています
    local current_widget="${widgets[zle-line-finish]}"
    local old_func=""

    if [[ "$current_widget" == "user:"* ]]; then
        # "user:_original_func" から関数名部分だけを抽出
        old_func="${current_widget#user:}"
    fi

    # Rust側に元の状態を伝える (builtin なら "builtin"、なければ "")
    # ここで内部コマンドを呼び出し、Rust側の構造体に保存させる
    __zsh_infinite_internal store zle-line-finish "${old_func:-${current_widget}}"
    
    # シェル変数にも保持（後続の _infinite_transient_prompt で使うため）
    export _INFINITE_OLD_WIDGET="$old_func"

    # 自前のウィジェットを登録
    zle -N zle-line-finish _infinite_transient_prompt
} >/dev/null 2>&1