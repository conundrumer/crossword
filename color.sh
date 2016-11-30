cat - | sed -E "s/(    [^\.].+)/$(tput bold)\1$(tput sgr0)/g"
