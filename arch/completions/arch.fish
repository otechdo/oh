complete -c arch -f -a "(cat /tmp/pkgs)"
complete -c arch -s i -l setup -d 'Setup a new arch'
complete -c arch -s h -l help -d 'Display the help message'
complete -c arch -s S -l install -d 'Install all selected packages'
complete -c arch -s d -l deps -d 'Install all selected packages as deps'
complete -c arch -s R -l uninstall -d 'Uninstall all selected packages'
complete -c arch -s M -l mirrors -d 'Update the arch mirrors'
complete -c arch -s C -l check -d 'Check for available arch update'
complete -c arch -s u -l update -d 'Update the system'
complete -c arch -s w -l wiki -d 'Show arch wiki'
complete -c arch -s m -l man -d 'Show arch manpages'
complete -c arch -s m -l woman -d 'Show arch manpages'
complete -c arch -s f -l forum -d 'Show arch forum'
complete -c arch -s a -l aur -d 'Show aur packages'
complete -c arch -s n -l news -d 'Show arch news'
complete -c arch -s c -l cache -d 'Refresh arch packages cache'
complete -c arch -s U -l upgrade -d 'Update arch and reboot after 5 min'
complete -c arch -s d -l download-updates -d 'Dowload all arch updates'
complete -c arch -s x -l cancel -d 'Cancel arch reboot'
complete -c arch -s v -l version -d 'Display arch version'
complete -c arch -s s -l search -d 'Search a package'
complete -c arch -l setup-new-config  -d "Clean current profile and relaunch setup"