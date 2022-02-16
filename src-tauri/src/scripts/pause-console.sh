#!/bin/bash
###
# Generate by vscch4. Edit it only if you know what you are doing!
if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <Executable> [<Arguments...>]"
    exit
fi
function set-title() {
    if [[ -z "$ORIG" ]]; then
        ORIG=$PS1
    fi
    TITLE="\[\e]2;$*\a\]"
    PS1=${ORIG}${TITLE}
}
set-title $1
start_time="$(date -u +%s.%4N)"
"$1" "${@:2}"
exit_code=$?
end_time="$(date -u +%s.%4N)"
elapsed_time="$(echo "$end_time-$start_time" | bc | sed 's/^\./0./')"

echo
echo -n "----------------"
RESET='\033[0m'
BG_RED='\033[41m'
BG_GREEN='\033[42m' 
BG_YELLOW_FG_BLACK='\033[43;30m'
FG_RED='\033[0;31m'
FG_GREEN='\033[0;32m'
FG_YELLOW='\033[0;33m'
# PowerLine Glyphs < and >
GT='\ue0b0'
LT='\ue0b2'
if [[ exit_code -eq 0 ]]; then
    exit_fg_color=$FG_GREEN
    exit_bg_color=$BG_GREEN
else
    exit_fg_color=$FG_RED
    exit_bg_color=$BG_RED
fi
echo -e -n "${exit_fg_color}${LT}${RESET}"
echo -e -n "${exit_bg_color} 返回值 ${exit_code} ${RESET}"
echo -e -n "${BG_YELLOW_FG_BLACK} 用时 ${elapsed_time}s ${RESET}"
echo -e -n "${FG_YELLOW}${GT}${RESET}"
echo "----------------"
read -n 1 -r -p "进程已退出。按任意键关闭窗口..."
