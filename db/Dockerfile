FROM flyway/flyway:latest

ENV DB_HOST=db
ENV DB_PORT=5432

COPY ./migration/ /flyway/sql/
COPY ./wait-for-it.sh ./wait-for-it.sh

RUN chmod +x ./wait-for-it.sh

SHELL ["/bin/bash", "-c"]
ENTRYPOINT exec ./wait-for-it.sh $DB_HOST:$DB_PORT -- flyway migrate
