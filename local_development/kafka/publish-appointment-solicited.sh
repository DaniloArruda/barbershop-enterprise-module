#!/bin/sh
echo "Publishing appointment solicited..."

message=`cat ./local_development/kafka/payloads/appointment-solicited.txt`;

command="echo $message | kafka-console-producer --bootstrap-server=kafka:29092 --topic appointment.solicited";

docker-compose exec kafka bash -c "${command}"
