#!/bin/bash

TTY_BOLD=$(tput bold)
TTY_NORMAL=$(tput sgr0)
TTY_RED=$(tput setaf 1)
TTY_GREEN=$(tput setaf 2)
TTY_YELLOW=$(tput setaf 3)
TTY_BLUE=$(tput setaf 4)

INSTALL_ERROR=false
LICER_VERSION="0.1.0"
INSTALL_DIR="${HOME}/.local/bin"
INSTALLED_VERSION=$(licer -v 2> /dev/null | awk '{ print substr($0, 8, 3) }')

function print() {
    case $2 in
        "info")
            printf "${TTY_BOLD}${TTY_BLUE}INFO${TTY_NORMAL} $1\n"
        ;;
        "success")
            printf "${TTY_BOLD}${TTY_GREEN}SUCCESS${TTY_NORMAL} $1\n"
        ;;
        "warning")
            printf "${TTY_BOLD}${TTY_YELLOW}WARN${TTY_NORMAL} $1\n"
        ;;
        "error")
            printf "${TTY_BOLD}${TTY_RED}ERROR${TTY_NORMAL} $1\n"
        ;;
    esac
}

case $(uname -m -s | awk '{ print tolower($0) }') in
    *"darwin arm64"*)
        LICER_OS="aarch64-apple-darwin"
    ;;
    *"darwin x86_64"*)
        LICER_OS="x86_64-apple-darwin"
    ;;
    *"linux arm64"*)
        LICER_OS="aarch64-unknown-linux-gnu"
    ;;
    *"linux x86_64"*)
        LICER_OS="x86_64-unknown-linux-gnu"
    ;;
    *)
        print "Licer is not precompiled for your system! You may clone and compile it from here: https://github.com/zahtec/licer" "error"
        exit 1
    ;;
esac

if [[ ${INSTALLED_VERSION//./} -ge ${LICER_VERSION//./} ]]; then
    echo "You are attempting to install ${TTY_BOLD}Licer v${LICER_VERSION}${TTY_NORMAL} when ${TTY_BOLD}Licer v${INSTALLED_VERSION}${TTY_NORMAL}, the same or later version, is already installed."

    read -p "Proceed? [Y/n] " RESPONSE

    if [[ $RESPONSE != "Y" ]]; then
        exit 0
    fi
fi

mkdir -p $INSTALL_DIR || {
    print "Failed to find or create ${INSTALL_DIR}!" "error"
    exit 1
}

print "Downloading and extracting Licer v${LICER_VERSION}..." "info"

curl -fsL "https://github.com/zahtec/licer/releases/download/v${LICER_VERSION}/licer-v${LICER_VERSION}-${LICER_OS}.tar.gz" | tar -xC $INSTALL_DIR || {
    print "Failed to download and extract Licer v${LICER_VERSION}" "error"
    exit 1
}

chmod u+rx "${INSTALL_DIR}/licer" || {
    print "Failed to make Licer v${LICER_VERSION} executable! Please run: chmod u+rx \"${INSTALL_DIR}/licer\"" "warning"
    INSTALL_ERROR=true
}

if [[ $PATH != *$INSTALL_DIR* ]]; then
    print "Please add ${INSTALL_DIR} to your PATH variable in order to utilize Licer globally!" "warning"
    INSTALL_ERROR=true
fi

if $INSTALL_ERROR; then
    print "Licer v${LICER_VERSION} installed with errors. ${TTY_BOLD}Please check above to ensure usage capabilities.${TTY_NORMAL}" "warning"
else
    print "Licer v${LICER_VERSION} installed successfully!" "success"
fi

exit 0
