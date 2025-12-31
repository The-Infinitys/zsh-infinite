#!/bin/zsh

# 1. 登録したフックを解除 (add-zsh-hook -d を使用)
autoload -Uz add-zsh-hook
add-zsh-hook -d precmd _update_infinite_prompt

# 3. 定義したシェル関数を削除してメモリを解放
unfunction _update_infinite_prompt
unfunction _infinite_transient_prompt
unfunction _reset_infinite_cursor


# 5. オプションを戻す（必要に応じて）
