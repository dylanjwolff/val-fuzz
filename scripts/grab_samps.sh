find . -maxdepth 1 -type d \( ! -name . \) -exec zsh -c 'cd {} && ls **/*.smt2 | shuf | head -250 | xargs -n 1 mv -t ../../samples' \;
