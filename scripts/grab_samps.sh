find . -maxdepth 1 -type d \( ! -name . \) -exec zsh -c 'cd {} && ls **/*.smt2 | shuf | head -2 ' \;
