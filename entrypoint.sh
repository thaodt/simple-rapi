#!/bin/bash

# Run SQL migrations
sqlx migrate run

# Start the application
exec app