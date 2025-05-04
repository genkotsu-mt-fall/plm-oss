#!/bin/bash
set -e

./tests/wait-for-backend.sh
./tests/part/api_test.sh
