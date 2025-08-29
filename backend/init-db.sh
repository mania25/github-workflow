#!/bin/bash
mkdir -p /tmp
touch /tmp/todos.db
chmod 664 /tmp/todos.db
exec /app/todo-backend