#!/usr/bin/env bash

./server >> backend.log & 
nginx -g 'daemon off;'