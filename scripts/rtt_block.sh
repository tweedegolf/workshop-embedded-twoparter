#/usr/bin/env bash

APP_NAME=$1
if [ -z $APP_NAME ]
then
  echo "Please specify your app's name: $0 <app_name>"
  exit -1
fi


rust-nm -S target/thumbv7*/debug/$APP_NAME | grep RTT | sed -re 's/^(.*) (.*) D _SEGGER_RTT$/RTT block info\nStart address:\t0x\1\nLength:\t\t0x\2/'
echo "Use this information to inform OpenOCD where it can find the RTT block in .vscode/launch.json"