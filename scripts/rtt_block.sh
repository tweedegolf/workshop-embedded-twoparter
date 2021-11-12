#/usr/bin/env bash

APP_NAME=$1
if [ -z $APP_NAME ]
then
  echo "Please specify your app's name: $0 <app_name>"
  exit -1
fi


rust-nm -S target/thumbv7*/debug/$APP_NAME | grep RTT