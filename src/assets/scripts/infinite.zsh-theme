#!/bin/zsh

# プロンプト内でのコマンド置換を有効化
setopt PROMPT_SUBST
ZLE_RPROMPT_INDENT=0
# 1. 通常時（入力中）のプロンプトを構築する関数
function set_full_prompt() {
    PROMPT='$( {{RUN_DIR}}/zsh-infinite zsh prompt left 2>/dev/null)'
    RPROMPT='$( {{RUN_DIR}}/zsh-infinite zsh prompt right 2>/dev/null)'
}

# 2. コマンド確定後（実行直前）に呼ばれるウィジェット
function zle-line-finish() {
    # 実行済みの行をシンプルなデザインに書き換える
    PROMPT='$( {{RUN_DIR}}/zsh-infinite zsh prompt transient --exit-code $? 2>/dev/null)'
    RPROMPT='' # 実行後は右プロンプトを消すとスッキリします
    zle reset-prompt
}
zle -N zle-line-finish

# 3. コマンド終了後、次のプロンプトを表示する前にフルデザインに戻す
function precmd() {
    set_full_prompt
}

# 初回のプロンプト設定
set_full_prompt
