complete -c arch -f -a "(cat /tmp/pkgs)"

complete -c arch -s c -l check -d 'Check for available updates'
complete -c arch -s l -l list -d 'Display the list of pending updates'
complete -c arch -s d -l devel -d 'Include AUR development packages updates'
complete -c arch -s n -l news -d 'Display latest Arch news'
complete -c arch -s D -l debug -d 'Display debug traces'
complete -c arch -s h -l help -d 'Display the help message'
complete -c arch -s S -l install -d 'install packages'