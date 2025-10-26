FROM valkey/valkey:8.1.3-bookworm
EXPOSE 6379
ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["valkey-server", "--tcp-backlog", "128"]
