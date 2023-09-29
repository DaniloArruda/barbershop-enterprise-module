#!/bin/sh
echo "Publishing appointment confirmed..."

message=`cat ./local_development/kafka/payloads/appointment-solicited.txt`; ## change the file to be "confirmed" further

command="echo $message | kafka-console-producer --bootstrap-server=kafka:29092 --topic appointment.confirmed";

docker compose exec kafka bash -c "${command}"
