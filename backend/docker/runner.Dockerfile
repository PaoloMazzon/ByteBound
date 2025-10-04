# This dockerfile effectively just adds an entrypoint script
# to alpine that you can pass arguments to through docker run
FROM alpine:latest

# Setup dependencies
RUN apk add --no-cache bash

# Copy entrypoint and make it runnable
COPY runner_entry.sh /usr/local/bin/runner_entry.sh
RUN chmod 777 /usr/local/bin/runner_entry.sh

# Setup entrypoints to passthrough parameters
ENTRYPOINT ["/usr/local/bin/runner_entry.sh"]
CMD [""]