# Base image
FROM denoland/deno:alpine

# Signals intent to devs for networking
EXPOSE 8080

# mkdir and cd
WORKDIR /usr/src/deno-api

# do not use root user
USER deno

# add all files from current local dir to WORKDIR
ADD . .
# cache dependencies (npm install equivalent for Deno)
RUN deno cache server.ts

# Run this command when we start the container:
CMD ["run", "--allow-net", "--no-check", "server.ts"]

# docker build -t deno-api .