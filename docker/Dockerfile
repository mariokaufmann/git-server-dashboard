FROM ubuntu:jammy
RUN apt update
RUN apt install -y libssl-dev ca-certificates
WORKDIR /app
COPY branch-dashboard /app/
COPY static /app/static
CMD ["/app/branch-dashboard"]
EXPOSE 8080
