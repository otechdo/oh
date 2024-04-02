complete -c arch -f

complete -c arch -s c -l check -d 'Check for available updates'
complete -c arch -s l -l list -d 'Display the list of pending updates'
complete -c arch -s d -l devel -d 'Include AUR development packages updates'
complete -c arch -s n -l news -d 'Display latest Arch news'
complete -c arch -s D -l debug -d 'Display debug traces'
complete -c arch -s h -l help -d 'Display the help message'
complete -c arch -s V -l version -d 'Display version information'
complete -c arch -s S -l install -d 'Install packages' -a "(cat /tmp/pkgs)"