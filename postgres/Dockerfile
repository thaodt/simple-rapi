FROM postgres:latest

COPY read_secrets.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/read_secrets.sh

ENTRYPOINT ["read_secrets.sh"]
CMD ["postgres"]