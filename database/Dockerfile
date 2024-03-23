FROM postgres:14.3

# Copy the SQL script to initialize the database
COPY init.sql /docker-entrypoint-initdb.d/

# This is a workaround for conditional copying
# If it's a test environment, the test SQL script is removed
# Copy the SQL with test data
COPY test-seed.sql /docker-entrypoint-initdb.d/

ARG ENV=development

RUN if [ "$ENV" != "development" ]; then \
    rm /docker-entrypoint-initdb.d/test-seed.sql; \
  else \
    echo "Seeding database with testing data"; \
  fi

