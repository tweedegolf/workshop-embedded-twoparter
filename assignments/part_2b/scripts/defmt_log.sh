#!/usr/bin/env bash

APP_NAME=$1
if [ -z $APP_NAME ]
then
  echo "Please specify your app's name: $0 <app_name>"
  exit -1
fi


echo "Listening to localhost:8765 for RTT logs. Press CTRL+C to quit."
nc localhost 8765 | defmt-print -e target/thumbv7*/debug/$APP_NAME