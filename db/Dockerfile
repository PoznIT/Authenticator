# Use official Flyway image as base
FROM flyway/flyway:latest
# Copy your SQL migration scripts into Flyway's sql folder
ENV DB_HOST=db
ENV DB_PORT=5432
COPY ./migration/ /flyway/sql/
COPY ./wait-for-it.sh ./wait-for-it.sh
RUN chmod +x ./wait-for-it.sh
ENTRYPOINT ./wait-for-it.sh $DB_HOST:$DB_PORT -- flyway migrate
