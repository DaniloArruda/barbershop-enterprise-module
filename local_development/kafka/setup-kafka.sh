#!/bin/sh
echo "Creating Kafka topics..."

topics="appointment.solicited
        appointment.confirmed"

for topic in $topics
do
    command="kafka-topics --if-not-exists \
                          --topic=${topic} \
                          --bootstrap-server=kafka:29092 \
                          --create"
    docker compose exec kafka bash -c "${command}"
done

# schemas="local_development/bin/kafka/schemas_local"

# for f in ${schemas}/*; do
#   topic=$(basename ${f} .avsc)
#   content=$(cat ${f})
#   echo "Processing Topic: $topic"
#   curl -s -X POST -H "Content-Type: application/vnd.schemaregistry.v1+json" \
#     --data "{\"schema\":${content}}" \
#     "http://localhost:8082/subjects/${topic}-value/versions" > /dev/null
# done
