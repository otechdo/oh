#compdef arch

local -a opts
opts=(
    {-i,--setup}'[Setup a new arch]'
    {-S,--install}'[Install all selected packages]'
    {-R,--uninstall}'[Uninstall all selected packages]'
    {-M,--mirrors}'[Update the arch mirrors]'
    {-C,--check}'[Check for available arch update]'
    {-u,--update}'[Update the system]'
    {-d,--deps}'[Install all selected packages as deps]'
    {-w,--wiki}'[Show arch wiki]'
	{-m,--man}'[Show arch manpages]'
	{-m,--woman}'[Show arch manpages]'
	{-f,--forum}'[Show arch forum]'
	{-a,--aur}'[Show aur packages]'
	{-n,--news}'[Show arch news]'
    {-c, --cache}'[Refresh arch packages cache]'
    {-U,--upgrade}'[Update arch and reboot after 5 min]'
    {-x,--cancel}'[Cancel arch reboot]'
    {-h,--help}'[Display the help message]'
    {-d,--download-updates}'['Dowload all arch updates]'
	{-v,--version}'[Display version information]'
)

_arguments $opts