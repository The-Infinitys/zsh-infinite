# --- dev.zshrc ---

# 共通設定: バイナリへのパスを通す
export PATH="{{RUN_DIR}}:$PATH"
alias zsh-infinite="{{RUN_DIR}}/zsh-infinite"
source "{{RUN_DIR}}/.zsh-theme"
