#!/usr/bin/env bash

echo -e '\x1b[34;1mDownloading...\x1b[0m'
wget 'https://github.com/Toxikuu/pact/releases/latest/download/pact' > /dev/null
chmod +x pact

echo -e '\x1b[34;1mInstalling...\x1b[0m'
sudo mv -vi pact /usr/bin/pact
echo -e '\x1b[34;1mDone!\x1b[0m'
